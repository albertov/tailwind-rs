use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRingOffsetColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindRingOffsetColor);

impl Display for TailwindRingOffsetColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ring-offset-{}", self.color)
    }
}

impl TailwindInstance for TailwindRingOffsetColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "--tw-ring-offset-color" => self.color.get_properties(ctx),
            "--tw-ring-offset-shadow" => format!("var(--tw-ring-inset, ) 0 0 0 var(--tw-ring-offset-width, 0px) {}", self.color.get_properties(ctx)),
            "box-shadow" => "var(--tw-inset-shadow, 0 0 #0000), var(--tw-inset-ring-shadow, 0 0 #0000), var(--tw-ring-offset-shadow), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow, 0 0 #0000)"
        }
    }
}
