use super::*;

impl SizingUnit {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        // Helper to create standard spacing values
        let spacing_px = |name: &str, value: f32| Ok(Self::SpacingValue(name.to_string(), LengthUnit::px(value)));
        let spacing_rem = |name: &str, value: f32| Ok(Self::SpacingValue(name.to_string(), LengthUnit::rem(value)));
        
        match pattern {
            ["min"] => Ok(Self::Min),
            ["max"] => Ok(Self::Max),
            ["auto"] => Ok(Self::Auto),
            ["full"] => Ok(Self::Full),
            ["fit"] => Ok(Self::Fit),
            ["screen"] => Ok(Self::Screen),
            ["xs"] => Ok(Self::Preset(PresetSize::Xs)),
            ["sm"] => Ok(Self::Preset(PresetSize::Sm)),
            ["md"] => Ok(Self::Preset(PresetSize::Md)),
            ["lg"] => Ok(Self::Preset(PresetSize::Lg)),
            ["xl"] => Ok(Self::Preset(PresetSize::Xl)),
            ["2xl"] => Ok(Self::Preset(PresetSize::Xxl)),
            ["3xl"] => Ok(Self::Preset(PresetSize::Xxxl)),
            ["4xl"] => Ok(Self::Preset(PresetSize::Xxxxl)),
            ["5xl"] => Ok(Self::Preset(PresetSize::Xxxxxl)),
            ["6xl"] => Ok(Self::Preset(PresetSize::Xxxxxxl)),
            ["7xl"] => Ok(Self::Preset(PresetSize::Xxxxxxxl)),
            // Spacing scale values - store the name for display
            ["0"] => spacing_px("0", 0.0),
            ["px"] => spacing_px("px", 1.0),
            ["0.5"] => spacing_rem("0.5", 0.125),
            ["1"] => spacing_rem("1", 0.25),
            ["1.5"] => spacing_rem("1.5", 0.375),
            ["2"] => spacing_rem("2", 0.5),
            ["2.5"] => spacing_rem("2.5", 0.625),
            ["3"] => spacing_rem("3", 0.75),
            ["3.5"] => spacing_rem("3.5", 0.875),
            ["4"] => spacing_rem("4", 1.0),
            ["5"] => spacing_rem("5", 1.25),
            ["6"] => spacing_rem("6", 1.5),
            ["7"] => spacing_rem("7", 1.75),
            ["8"] => spacing_rem("8", 2.0),
            ["9"] => spacing_rem("9", 2.25),
            ["10"] => spacing_rem("10", 2.5),
            ["11"] => spacing_rem("11", 2.75),
            ["12"] => spacing_rem("12", 3.0),
            ["14"] => spacing_rem("14", 3.5),
            ["16"] => spacing_rem("16", 4.0),
            ["20"] => spacing_rem("20", 5.0),
            ["24"] => spacing_rem("24", 6.0),
            ["28"] => spacing_rem("28", 7.0),
            ["32"] => spacing_rem("32", 8.0),
            ["36"] => spacing_rem("36", 9.0),
            ["40"] => spacing_rem("40", 10.0),
            ["44"] => spacing_rem("44", 11.0),
            ["48"] => spacing_rem("48", 12.0),
            ["52"] => spacing_rem("52", 13.0),
            ["56"] => spacing_rem("56", 14.0),
            ["60"] => spacing_rem("60", 15.0),
            ["64"] => spacing_rem("64", 16.0),
            ["68"] => spacing_rem("68", 17.0),
            ["72"] => spacing_rem("72", 18.0),
            ["76"] => spacing_rem("76", 19.0),
            ["80"] => spacing_rem("80", 20.0),
            ["84"] => spacing_rem("84", 21.0),
            ["88"] => spacing_rem("88", 22.0),
            ["92"] => spacing_rem("92", 23.0),
            ["96"] => spacing_rem("96", 24.0),
            [n] => Self::parse_arbitrary(&TailwindArbitrary::from(*n)),
            [] => Self::parse_arbitrary(arbitrary),
            _ => syntax_error!("Unknown sizing instructions: {}", pattern.join("-")),
        }
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Self::maybe_fraction(arbitrary).or_else(|_| Self::maybe_no_unit(arbitrary)).or_else(|_| Self::maybe_length(arbitrary))
    }
    #[inline]
    fn maybe_length(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Length(arbitrary.as_length_or_fraction()?))
    }
    #[inline]
    fn maybe_no_unit(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let rem = |x| Ok(Self::Length(LengthUnit::em(x)));
        rem(arbitrary.as_float()? / 4.0)
    }
    #[inline]
    fn maybe_fraction(arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (a, b) = arbitrary.as_fraction()?;
        Ok(Self::Fraction(a, b))
    }
}

impl TailwindSizing {
    #[inline]
    pub fn parse_width(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Width, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_min(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinWidth, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_width_max(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxWidth, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_height(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::Height, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_min(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MinHeight, size: SizingUnit::parse(pattern, arbitrary)? })
    }
    #[inline]
    pub fn parse_height_max(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: TailwindSizingKind::MaxHeight, size: SizingUnit::parse(pattern, arbitrary)? })
    }
}
