mod builder;
mod display;
pub mod size;

use super::*;
pub use size::TailwindSize;

#[derive(Copy, Clone, Debug)]
enum TailwindSizingKind {
    Width,
    MinWidth,
    MaxWidth,
    Height,
    MinHeight,
    MaxHeight,
}

#[derive(Copy, Clone, Debug)]
enum PresetSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
    Xxxxl,
    Xxxxxl,
    Xxxxxxl,
    Xxxxxxxl,
}

#[derive(Clone, Debug)]
enum SizingUnit {
    Min,
    Max,
    Fit,
    Auto,
    Full,
    Screen,
    Preset(PresetSize),
    Fraction(usize, usize),
    // Standard spacing values (0, px, 0.5, 1, 2, 4, etc.)
    SpacingValue(String, LengthUnit),
    // Arbitrary values wrapped in brackets
    Length(LengthUnit),
}

#[doc = include_str!("sizing.md")]
#[derive(Clone, Debug)]
pub struct TailwindSizing {
    kind: TailwindSizingKind,
    size: SizingUnit,
}
