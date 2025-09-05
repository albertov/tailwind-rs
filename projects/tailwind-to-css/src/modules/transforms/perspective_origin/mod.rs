use crate::{css_attributes, AnchorPoint, CssAttributes, Result, TailwindArbitrary, TailwindBuilder, TailwindInstance};
use std::fmt::{Debug, Display, Formatter};

/// Perspective origin utility
/// 
/// Supports all standard positions:
/// - perspective-origin-center
/// - perspective-origin-top
/// - perspective-origin-bottom
/// - perspective-origin-left
/// - perspective-origin-right
/// - perspective-origin-top-left
/// - perspective-origin-top-right
/// - perspective-origin-bottom-left
/// - perspective-origin-bottom-right
#[derive(Clone, Debug)]
pub struct TailwindPerspectiveOrigin {
    anchor: AnchorPoint,
}

impl Display for TailwindPerspectiveOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "perspective-origin-{}", self.anchor.get_class())
    }
}

impl TailwindInstance for TailwindPerspectiveOrigin {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "perspective-origin" => self.anchor.get_properties(),
        }
    }
}

impl TailwindPerspectiveOrigin {
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let anchor = AnchorPoint::parse(input, arbitrary, true)?;
        Ok(Self { anchor })
    }
}