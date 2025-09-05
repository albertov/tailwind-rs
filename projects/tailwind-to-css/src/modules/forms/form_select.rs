use crate::{css_attributes, CssAttributes, TailwindBuilder, TailwindInstance};
use std::fmt::{Display, Formatter};

/// Form select utility for styling select dropdowns
/// https://github.com/tailwindlabs/tailwindcss-forms
#[derive(Debug, Clone)]
pub struct TailwindFormSelect;

impl Display for TailwindFormSelect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("form-select")
    }
}

impl TailwindInstance for TailwindFormSelect {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "appearance" => "none",
            "background-color" => "#ffffff",
            "border-color" => "#d1d5db",
            "border-width" => "1px",
            "border-radius" => "0.375rem",
            "padding-top" => "0.5rem",
            "padding-right" => "2.5rem",
            "padding-bottom" => "0.5rem",
            "padding-left" => "0.75rem",
            "font-size" => "1rem",
            "line-height" => "1.5rem",
            "color" => "#111827",
            "background-image" => "url(\"data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3e%3c/svg%3e\")",
            "background-position" => "right 0.5rem center",
            "background-repeat" => "no-repeat",
            "background-size" => "1.5em 1.5em"
        }
    }
}

impl TailwindFormSelect {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Default for TailwindFormSelect {
    fn default() -> Self {
        Self::new()
    }
}