use std::fmt::{Debug, Display, Formatter};

use tailwind_error::Result;

use crate::{CssAttributes, NumericValue, TailwindArbitrary, TailwindBuilder, TailwindInstance};

pub use self::{
    blur::TailwindBlur, brightness::TailwindBrightness, contrast::TailwindContrast, grayscale::TailwindGrayscale,
    hue_rotate::TailwindHueRotate, invert::TailwindInvert, saturate::TailwindSaturate, sepia::TailwindSepia,
};

mod blur;
mod brightness;
mod contrast;
mod grayscale;
mod hue_rotate;
mod invert;
mod saturate;
mod sepia;

#[derive(Clone, Debug)]
pub(crate) struct Backdrop(pub(crate) bool);

impl From<bool> for Backdrop {
    fn from(backdrop: bool) -> Self {
        Self(backdrop)
    }
}

impl Backdrop {
    pub fn write(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            true => {
                write!(f, "backdrop-")
            },
            false => {
                write!(f, "")
            },
        }
    }
    
    /// Get filter attributes using CSS variables for composition
    /// Each filter sets its own CSS variable and the complete filter chain
    pub fn get_filter_var<T>(&self, var_name: &str, value: T) -> CssAttributes
    where
        T: Into<String>,
    {
        let mut css = CssAttributes::default();
        let value_str = value.into();
        
        // Set only the specific variable for this filter
        // The preflight system handles initialization of all CSS variables
        let filter_chain = if self.0 {
            css.insert(format!("--tw-backdrop-{}", var_name), value_str);
            "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia) var(--tw-backdrop-drop-shadow)"
        } else {
            css.insert(format!("--tw-{}", var_name), value_str);
            "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-drop-shadow) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-opacity) var(--tw-saturate) var(--tw-sepia)"
        };
        
        // Set the filter property with the complete chain
        match self.0 {
            true => css.insert("backdrop-filter", filter_chain),
            false => css.insert("filter", filter_chain),
        }
        
        css
    }
    
    #[allow(dead_code)]
    pub fn get_filter<T>(&self, value: T) -> CssAttributes
    where
        T: Into<String>,
    {
        let mut css = CssAttributes::default();
        match self.0 {
            true => css.insert("backdrop-filter", value.into()),
            false => css.insert("filter", value.into()),
        }
        css
    }
    pub fn get_opacity<T>(&self, value: T) -> CssAttributes
    where
        T: Into<String>,
    {
        let mut css = CssAttributes::default();
        match self.0 {
            true => css.insert("backdrop-filter", format!("opacity({})", value.into())),
            false => css.insert("opacity", value.into()),
        }
        css
    }
    pub fn get_shadow<T>(&self, value: T) -> CssAttributes
    where
        T: Into<String>,
    {
        let mut css = CssAttributes::default();
        match self.0 {
            true => css.insert("filter", value.into()),
            false => css.insert("box-shadow", value.into()),
        }
        css
    }
}
