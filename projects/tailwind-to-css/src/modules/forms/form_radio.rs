use crate::{css_attributes, CssAttributes, TailwindBuilder, TailwindInstance};
use std::fmt::{Display, Formatter};

/// Form radio utility for styling radio inputs
/// https://github.com/tailwindlabs/tailwindcss-forms
#[derive(Debug, Clone)]
pub struct TailwindFormRadio;

impl Display for TailwindFormRadio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("form-radio")
    }
}

impl TailwindInstance for TailwindFormRadio {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "appearance" => "none",
            "padding" => "0",
            "print-color-adjust" => "exact",
            "display" => "inline-block",
            "vertical-align" => "middle",
            "background-origin" => "border-box",
            "user-select" => "none",
            "flex-shrink" => "0",
            "height" => "1rem",
            "width" => "1rem",
            "color" => "#3b82f6",
            "background-color" => "#ffffff",
            "border-color" => "#d1d5db",
            "border-width" => "1px",
            "border-radius" => "100%"
        }
    }
}

impl TailwindFormRadio {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Default for TailwindFormRadio {
    fn default() -> Self {
        Self::new()
    }
}