use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBackgroundPosition {
    kind: AnchorPoint,
}

impl Display for TailwindBackgroundPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let class = match &self.kind {
            AnchorPoint::LeftTop => "left-top",
            AnchorPoint::Top => "top",
            AnchorPoint::RightTop => "right-top",
            AnchorPoint::Left => "left",
            AnchorPoint::Center => "center",
            AnchorPoint::Right => "right",
            AnchorPoint::LeftBottom => "left-bottom",
            AnchorPoint::Bottom => "bottom",
            AnchorPoint::RightBottom => "right-bottom",
            AnchorPoint::Standard(s) => s,
            AnchorPoint::Arbitrary(a) => return write!(f, "bg-{}", a.get_class()),
        };
        write!(f, "bg-{}", class)
    }
}

impl TailwindInstance for TailwindBackgroundPosition {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "background-position" => self.kind.get_properties()
        }
    }
}

impl TailwindBackgroundPosition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: AnchorPoint::parse(pattern, arbitrary, true)? })
    }
}
