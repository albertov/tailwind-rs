use super::*;

/// Inset ring color utilities (inset-ring-red-500, inset-ring-blue-600, etc.)
#[derive(Clone, Debug)]
pub struct TailwindInsetRingColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindInsetRingColor);

impl Display for TailwindInsetRingColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "inset-ring-{}", self.color)
    }
}

impl TailwindInstance for TailwindInsetRingColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "--tw-inset-ring-color" => self.color.get_properties(ctx),
            "--tw-inset-ring-shadow" => format!("inset 0 0 0 var(--tw-inset-ring-width, 1px) {}", self.color.get_properties(ctx)),
            "box-shadow" => "var(--tw-inset-shadow, 0 0 #0000), var(--tw-inset-ring-shadow), var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow, 0 0 #0000)"
        }
    }
}