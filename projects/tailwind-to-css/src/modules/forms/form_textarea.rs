use crate::{css_attributes, CssAttributes, TailwindBuilder, TailwindInstance};
use std::fmt::{Display, Formatter};

/// Form textarea utility for styling textarea elements
/// https://github.com/tailwindlabs/tailwindcss-forms
#[derive(Debug, Clone)]
pub struct TailwindFormTextarea;

impl Display for TailwindFormTextarea {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("form-textarea")
    }
}

impl TailwindInstance for TailwindFormTextarea {
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
            "color" => "#111827",
            "resize" => "vertical"
        }
    }
}

impl TailwindFormTextarea {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Default for TailwindFormTextarea {
    fn default() -> Self {
        Self::new()
    }
}