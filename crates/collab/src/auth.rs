use crate::{
    AppState, Error, Result,
    db::{self, AccessTokenId, Database, UserId},
    rpc::Principal,
};
use anyhow::Context as _;
use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use base64::prelude::*;
use prometheus::{Histogram, exponential_buckets, register_histogram};
use rand::Rng;
pub use rpc::auth::random_token;
use scrypt::{
    Scrypt,
    password_hash::{PasswordHash, PasswordVerifier},
};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::sync::OnceLock;
use std::{sync::Arc, time::Instant};
use subtle::ConstantTimeEq;

/// Validates the authorization header and adds an Extension<Principal> to the request.
/// Authorization: <user-id> <token>
///   <token> can be an access_token attached to that user, or an access token of an admin
///   or (in development) the string ADMIN:<config.api_token>.
/// Authorization: "dev-server-token" <token>
pub async fn validate_header<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let mut auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::http(
                StatusCode::UNAUTHORIZED,
                "missing authorization header".to_string(),
            )
        })?
        .split_whitespace();

    let state = req.extensions().get::<Arc<AppState>>().unwrap();

    let first = auth_header.next().unwrap_or("");
    if first == "dev-server-token" {
        Err(Error::http(
            StatusCode::UNAUTHORIZED,
            "Dev servers were removed in Zed 0.157 please upgrade to SSH remoting".to_string(),
        ))?;
    }

    let user_id = UserId(first.parse().map_err(|_| {
        Error::http(
            StatusCode::BAD_REQUEST,
            "missing user id in authorization header".to_string(),
        )
    })?);

    let access_token = auth_header.next().ok_or_else(|| {
        Error::http(
            StatusCode::BAD_REQUEST,
            "missing access token in authorization header".to_string(),
        )
    })?;

    // In development, allow impersonation using the admin API token.
    // Don't allow this in production because we can't tell who is doing
    // the impersonating.
    let validate_result = if let (Some(admin_token), true) = (
        access_token.strip_prefix("ADMIN_TOKEN:"),
        state.config.is_development(),
    ) {
        Ok(VerifyAccessTokenResult {
            is_valid: state.config.api_token == admin_token,
            impersonator_id: None,
        })
    } else {
        verify_access_token(access_token, user_id, &state.db).await
    };

    if let Ok(validate_result) = validate_result
        && validate_result.is_valid
    {
        let user = state
            .db
            .get_user_by_id(user_id)
            .await?
            .with_context(|| format!("user {user_id} not found"))?;

        if let Some(impersonator_id) = validate_result.impersonator_id {
            let admin = state
                .db
                .get_user_by_id(impersonator_id)
                .await?
                .with_context(|| format!("user {impersonator_id} not found"))?;
            req.extensions_mut()
                .insert(Principal::Impersonated { user, admin });
        } else {
            req.extensions_mut().insert(Principal::User(user));
        };
        return Ok::<_, Error>(next.run(req).await);
    }

    Err(Error::http(
        StatusCode::UNAUTHORIZED,
        "invalid credentials".to_string(),
    ))
}

pub const MAX_ACCESS_TOKENS_TO_STORE: usize = 8;

#[derive(Serialize, Deserialize)]
pub struct AccessTokenJson {
    pub version: usize,
    pub id: AccessTokenId,
    pub token: String,
}

/// Creates a new access token to identify the given user. before returning it, you should
/// encrypt it with the user's public key.
pub async fn create_access_token(
    db: &db::Database,
    user_id: UserId,
    impersonated_user_id: Option<UserId>,
) -> Result<String> {
    const VERSION: usize = 1;
    let access_token = rpc::auth::random_token();
    let access_token_hash = generate_access_token_hash(&access_token);
    let id = db
        .create_access_token(
            user_id,
            impersonated_user_id,
            &access_token_hash,
            MAX_ACCESS_TOKENS_TO_STORE,
        )
        .await?;
    Ok(serde_json::to_string(&AccessTokenJson {
        version: VERSION,
        id,
        token: access_token,
    })?)
}

/// Hashing prevents anyone with access to the database being able to login.
/// As the token is randomly generated, we don't need to worry about scrypt-style
/// protection.
pub fn generate_access_token_hash(token: &str) -> String {
    let mut rng = rand::rng();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt);
    let digest = compute_salted_hash(token, &salt);
    format!(
        "$sha256s${}${}",
        BASE64_URL_SAFE.encode(salt),
        BASE64_URL_SAFE.encode(digest)
    )
}

fn compute_salted_hash(token: &str, salt: &[u8]) -> Vec<u8> {
    let mut hasher = sha2::Sha256::new();
    hasher.update(salt);
    hasher.update(token.as_bytes());
    hasher.finalize().to_vec()
}

#[deprecated(note = "use generate_access_token_hash instead")]
pub fn hash_access_token(token: &str) -> String {
    let digest = sha2::Sha256::digest(token);
    format!("$sha256${}", BASE64_URL_SAFE.encode(digest))
}

/// Encrypts the given access token with the given public key to avoid leaking it on the way
/// to the client.
pub fn encrypt_access_token(access_token: &str, public_key: String) -> Result<String> {
    use rpc::auth::EncryptionFormat;

    /// The encryption format to use for the access token.
    const ENCRYPTION_FORMAT: EncryptionFormat = EncryptionFormat::V1;

    let native_app_public_key =
        rpc::auth::PublicKey::try_from(public_key).context("failed to parse app public key")?;
    let encrypted_access_token = native_app_public_key
        .encrypt_string(access_token, ENCRYPTION_FORMAT)
        .context("failed to encrypt access token with public key")?;
    Ok(encrypted_access_token)
}

pub struct VerifyAccessTokenResult {
    pub is_valid: bool,
    pub impersonator_id: Option<UserId>,
}

/// Checks that the given access token is valid for the given user.
pub async fn verify_access_token(
    token: &str,
    user_id: UserId,
    db: &Arc<Database>,
) -> Result<VerifyAccessTokenResult> {
    static METRIC_ACCESS_TOKEN_HASHING_TIME: OnceLock<Histogram> = OnceLock::new();
    let metric_access_token_hashing_time = METRIC_ACCESS_TOKEN_HASHING_TIME.get_or_init(|| {
        register_histogram!(
            "access_token_hashing_time",
            "time spent hashing access tokens",
            exponential_buckets(10.0, 2.0, 10).unwrap(),
        )
        .unwrap()
    });

    let token: AccessTokenJson = serde_json::from_str(token)?;

    let db_token = db.get_access_token(token.id).await?;
    let token_user_id = db_token.impersonated_user_id.unwrap_or(db_token.user_id);
    if token_user_id != user_id {
        return Err(anyhow::anyhow!("no such access token"))?;
    }
    let t0 = Instant::now();

    let mut needs_upgrade = false;
    let is_valid = if db_token.hash.starts_with("$scrypt$") {
        needs_upgrade = true;
        let db_hash = PasswordHash::new(&db_token.hash).map_err(anyhow::Error::new)?;
        Scrypt
            .verify_password(token.token.as_bytes(), &db_hash)
            .is_ok()
    } else if db_token.hash.starts_with("$sha256s$") {
        let parts: Vec<&str> = db_token.hash.split('$').collect();
        if parts.len() == 4 {
            let salt = BASE64_URL_SAFE.decode(parts[2]).unwrap_or_default();
            let stored_digest = BASE64_URL_SAFE.decode(parts[3]).unwrap_or_default();
            let computed_digest = compute_salted_hash(&token.token, &salt);
            stored_digest.ct_eq(&computed_digest).into()
        } else {
            false
        }
    } else if db_token.hash.starts_with("$sha256$") {
        needs_upgrade = true;
        #[allow(deprecated)]
        let token_hash = hash_access_token(&token.token);
        db_token.hash.as_bytes().ct_eq(token_hash.as_ref()).into()
    } else {
        false
    };

    let duration = t0.elapsed();
    log::info!("hashed access token in {:?}", duration);
    metric_access_token_hashing_time.observe(duration.as_millis() as f64);

    if is_valid && needs_upgrade {
        let new_hash = generate_access_token_hash(&token.token);
        db.update_access_token_hash(db_token.id, &new_hash).await?;
    }

    Ok(VerifyAccessTokenResult {
        is_valid,
        impersonator_id: if db_token.impersonated_user_id.is_some() {
            Some(db_token.user_id)
        } else {
            None
        },
    })
}
