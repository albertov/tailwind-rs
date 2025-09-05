use crate::{css_attributes, CssAttributes, TailwindBuilder, TailwindInstance};
use std::fmt::{Display, Formatter};

/// Form input utility for styling text inputs, email, password, number, date, etc.
/// https://github.com/tailwindlabs/tailwindcss-forms
#[derive(Debug, Clone)]
pub struct TailwindFormInput;

impl Display for TailwindFormInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("form-input")
    }
}

impl TailwindInstance for TailwindFormInput {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "appearance" => "none",
            "background-color" => "#ffffff",
            "border-color" => "#d1d5db",
            "border-width" => "1px",
            "border-radius" => "0.375rem",
            "padding-top" => "0.5rem",
            "padding-right" => "0.75rem",
            "padding-bottom" => "0.5rem",
            "padding-left" => "0.75rem",
            "font-size" => "1rem",
            "line-height" => "1.5rem",
            "color" => "#111827"
        }
    }
}

impl TailwindFormInput {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Default for TailwindFormInput {
    fn default() -> Self {
        Self::new()
    }
}