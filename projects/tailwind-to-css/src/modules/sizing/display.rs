use super::*;

impl Display for PresetSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Xs => write!(f, "xs"),
            Self::Sm => write!(f, "sm"),
            Self::Md => write!(f, "md"),
            Self::Lg => write!(f, "lg"),
            Self::Xl => write!(f, "xl"),
            Self::Xxl => write!(f, "2xl"),
            Self::Xxxl => write!(f, "3xl"),
            Self::Xxxxl => write!(f, "4xl"),
            Self::Xxxxxl => write!(f, "5xl"),
            Self::Xxxxxxl => write!(f, "6xl"),
            Self::Xxxxxxxl => write!(f, "7xl"),
        }
    }
}

impl Display for SizingUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Min => write!(f, "min"),
            Self::Max => write!(f, "max"),
            Self::Fit => write!(f, "fit"),
            Self::Auto => write!(f, "auto"),
            Self::Full => write!(f, "full"),
            Self::Screen => write!(f, "screen"),
            Self::Preset(preset) => write!(f, "{}", preset),
            Self::Fraction(numerator, denominator) => write!(f, "{}/{}", numerator, denominator),
            Self::Length(x) => write!(f, "[{}]", x),
        }
    }
}

impl PresetSize {
    fn to_rem(&self) -> f32 {
        match self {
            Self::Xs => 20.0,      // 320px
            Self::Sm => 24.0,      // 384px
            Self::Md => 28.0,      // 448px
            Self::Lg => 32.0,      // 512px
            Self::Xl => 36.0,      // 576px
            Self::Xxl => 42.0,     // 672px
            Self::Xxxl => 48.0,    // 768px
            Self::Xxxxl => 56.0,   // 896px
            Self::Xxxxxl => 64.0,  // 1024px
            Self::Xxxxxxl => 72.0, // 1152px
            Self::Xxxxxxxl => 80.0, // 1280px
        }
    }
}

impl SizingUnit {
    fn get_attribute(&self, is_width: bool) -> String {
        let is_width = match is_width {
            true => "vw",
            false => "vh",
        };
        match self {
            Self::Min => "min-content".to_string(),
            Self::Max => "max-content".to_string(),
            Self::Fit => "fit-content".to_string(),
            Self::Auto => "auto".to_string(),
            Self::Full => "100%".to_string(),
            Self::Screen => format!("100{}", is_width),
            Self::Preset(preset) => format!("{}rem", preset.to_rem()),
            Self::Fraction(numerator, denominator) => format!("{}%", *numerator as f32 / *denominator as f32),
            Self::Length(x) => format!("{}", x),
        }
    }
}

impl TailwindSizingKind {
    fn is_width(self) -> bool {
        matches!(self, Self::Width | Self::MinWidth | Self::MaxWidth)
    }
}

impl Display for TailwindSizingKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Width => f.write_str("width"),
            Self::MinWidth => f.write_str("min-width"),
            Self::MaxWidth => f.write_str("max-width"),
            Self::Height => f.write_str("height"),
            Self::MinHeight => f.write_str("min-height"),
            Self::MaxHeight => f.write_str("max-height"),
        }
    }
}

impl Display for TailwindSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.kind, self.size)
    }
}

impl TailwindInstance for TailwindSizing {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let class = self.kind.to_string();
        let width = self.size.get_attribute(self.kind.is_width());
        css_attributes! {
            class => width
        }
    }
}
