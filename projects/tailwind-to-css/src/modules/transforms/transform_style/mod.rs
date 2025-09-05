use crate::{css_attributes, CssAttributes, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};
use std::fmt::{Debug, Display, Formatter};

/// Transform style utility
/// 
/// Supports:
/// - transform-flat (transform-style: flat)
/// - transform-preserve-3d (transform-style: preserve-3d)
#[derive(Clone, Debug)]
pub struct TailwindTransformStyle {
    style: TransformStyleKind,
}

#[derive(Clone, Debug)]
enum TransformStyleKind {
    Flat,
    Preserve3d,
}

impl Display for TailwindTransformStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.style {
            TransformStyleKind::Flat => write!(f, "transform-flat"),
            TransformStyleKind::Preserve3d => write!(f, "transform-preserve-3d"),
        }
    }
}

impl TailwindInstance for TailwindTransformStyle {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let value = match self.style {
            TransformStyleKind::Flat => "flat",
            TransformStyleKind::Preserve3d => "preserve-3d",
        };
        css_attributes! {
            "transform-style" => value,
        }
    }
}

impl TailwindTransformStyle {
    pub fn parse(input: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        let style = match input {
            ["flat"] => TransformStyleKind::Flat,
            ["preserve", "3d"] => TransformStyleKind::Preserve3d,
            _ => return Err(().into()),
        };
        Ok(Self { style })
    }
}