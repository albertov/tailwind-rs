use crate::{css_attributes, CssAttributes, TailwindBuilder, TailwindInstance};
use std::fmt::{Display, Formatter};

/// Form checkbox utility for styling checkbox inputs
/// https://github.com/tailwindlabs/tailwindcss-forms
#[derive(Debug, Clone)]
pub struct TailwindFormCheckbox;

impl Display for TailwindFormCheckbox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("form-checkbox")
    }
}

impl TailwindInstance for TailwindFormCheckbox {
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
            "border-radius" => "0.25rem"
        }
    }
}

impl TailwindFormCheckbox {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Default for TailwindFormCheckbox {
    fn default() -> Self {
        Self::new()
    }
}