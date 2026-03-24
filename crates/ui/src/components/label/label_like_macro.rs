#[macro_export]
macro_rules! impl_label_common {
    ($type:ident) => {
        impl LabelCommon for $type {
            fn size(mut self, size: LabelSize) -> Self {
                self.base = self.base.size(size);
                self
            }

            fn weight(mut self, weight: gpui::FontWeight) -> Self {
                self.base = self.base.weight(weight);
                self
            }

            fn line_height_style(mut self, line_height_style: crate::LineHeightStyle) -> Self {
                self.base = self.base.line_height_style(line_height_style);
                self
            }

            fn color(mut self, color: Color) -> Self {
                self.base = self.base.color(color);
                self
            }

            fn strikethrough(mut self) -> Self {
                self.base = self.base.strikethrough();
                self
            }

            fn italic(mut self) -> Self {
                self.base = self.base.italic();
                self
            }

            fn alpha(mut self, alpha: f32) -> Self {
                self.base = self.base.alpha(alpha);
                self
            }

            fn underline(mut self) -> Self {
                self.base = self.base.underline();
                self
            }

            fn truncate(mut self) -> Self {
                self.base = self.base.truncate();
                self
            }

            fn single_line(mut self) -> Self {
                self.base = self.base.single_line();
                self
            }

            fn buffer_font(mut self, cx: &gpui::App) -> Self {
                self.base = self.base.buffer_font(cx);
                self
            }

            fn inline_code(mut self, cx: &gpui::App) -> Self {
                self.base = self.base.inline_code(cx);
                self
            }
        }
    };
}
