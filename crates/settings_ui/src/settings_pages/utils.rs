use settings::{AudioInputDeviceName, AudioOutputDeviceName, SettingsContent, ShowScrollbar};

pub(crate) const DEFAULT_STRING: String = String::new();
/// A default empty string reference. Useful in `pick` functions for cases either in dynamic item fields, or when dealing with `settings::Maybe`
/// to avoid the "NO DEFAULT" case.
pub(crate) const DEFAULT_EMPTY_STRING: Option<&String> = Some(&DEFAULT_STRING);

pub(crate) const DEFAULT_AUDIO_OUTPUT: AudioOutputDeviceName = AudioOutputDeviceName(None);
pub(crate) const DEFAULT_EMPTY_AUDIO_OUTPUT: Option<&AudioOutputDeviceName> = Some(&DEFAULT_AUDIO_OUTPUT);
pub(crate) const DEFAULT_AUDIO_INPUT: AudioInputDeviceName = AudioInputDeviceName(None);
pub(crate) const DEFAULT_EMPTY_AUDIO_INPUT: Option<&AudioInputDeviceName> = Some(&DEFAULT_AUDIO_INPUT);

macro_rules! concat_sections {
    (@vec, $($arr:expr),+ $(,)?) => {{
        let total_len = 0_usize $(+ $arr.len())+;
        let mut out = Vec::with_capacity(total_len);

        $(
            out.extend($arr);
        )+

        out
    }};

    ($($arr:expr),+ $(,)?) => {{
        let total_len = 0_usize $(+ $arr.len())+;

        let mut out: Box<[std::mem::MaybeUninit<_>]> = Box::new_uninit_slice(total_len);

        let mut index = 0usize;
        $(
            let array = $arr;
            for item in array {
                out[index].write(item);
                index += 1;
            }
        )+

        debug_assert_eq!(index, total_len);

        // SAFETY: we wrote exactly `total_len` elements.
        unsafe { out.assume_init() }
    }};
}

pub(crate) use concat_sections;

pub(crate) fn show_scrollbar_or_editor(
    settings_content: &SettingsContent,
    show: fn(&SettingsContent) -> Option<&ShowScrollbar>,
) -> Option<&ShowScrollbar> {
    show(settings_content).or(settings_content
        .editor
        .scrollbar
        .as_ref()
        .and_then(|scrollbar| scrollbar.show.as_ref()))
}

pub(crate) fn dynamic_variants<T>() -> &'static [T::Discriminant]
where
    T: strum::IntoDiscriminant,
    T::Discriminant: strum::VariantArray,
{
    <<T as strum::IntoDiscriminant>::Discriminant as strum::VariantArray>::VARIANTS
}
