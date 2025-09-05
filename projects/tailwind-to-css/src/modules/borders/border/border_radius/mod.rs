use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindRounded {
    kind: RoundedKind,
    size: RoundedSize,
}

#[derive(Clone, Debug)]
enum RoundedSize {
    None,
    Xs,
    Sm,
    Default, // When no size specified, defaults to sm
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Full,
    Arbitrary(LengthUnit),
}

// Includes support for logical properties
#[derive(Copy, Clone, Debug)]
enum RoundedKind {
    Rounded,
    RoundedT,
    RoundedR,
    RoundedB,
    RoundedL,
    RoundedTL,
    RoundedTR,
    RoundedBL,
    RoundedBR,
    // Logical properties
    RoundedS,   // start (inline-start)
    RoundedE,   // end (inline-end)
    RoundedSS,  // start-start corner
    RoundedSE,  // start-end corner
    RoundedES,  // end-start corner
    RoundedEE,  // end-end corner
}

impl Display for RoundedKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rounded => write!(f, "rounded"),
            Self::RoundedT => write!(f, "rounded-t"),
            Self::RoundedR => write!(f, "rounded-r"),
            Self::RoundedB => write!(f, "rounded-b"),
            Self::RoundedL => write!(f, "rounded-l"),
            Self::RoundedTL => write!(f, "rounded-tl"),
            Self::RoundedTR => write!(f, "rounded-tr"),
            Self::RoundedBL => write!(f, "rounded-bl"),
            Self::RoundedBR => write!(f, "rounded-br"),
            // Logical properties
            Self::RoundedS => write!(f, "rounded-s"),
            Self::RoundedE => write!(f, "rounded-e"),
            Self::RoundedSS => write!(f, "rounded-ss"),
            Self::RoundedSE => write!(f, "rounded-se"),
            Self::RoundedES => write!(f, "rounded-es"),
            Self::RoundedEE => write!(f, "rounded-ee"),
        }
    }
}

impl Display for TailwindRounded {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.size {
            RoundedSize::Default => write!(f, "{}", self.kind),
            RoundedSize::None => write!(f, "{}-none", self.kind),
            RoundedSize::Xs => write!(f, "{}-xs", self.kind),
            RoundedSize::Sm => write!(f, "{}-sm", self.kind),
            RoundedSize::Md => write!(f, "{}-md", self.kind),
            RoundedSize::Lg => write!(f, "{}-lg", self.kind),
            RoundedSize::Xl => write!(f, "{}-xl", self.kind),
            RoundedSize::Xl2 => write!(f, "{}-2xl", self.kind),
            RoundedSize::Xl3 => write!(f, "{}-3xl", self.kind),
            RoundedSize::Xl4 => write!(f, "{}-4xl", self.kind),
            RoundedSize::Full => write!(f, "{}-full", self.kind),
            RoundedSize::Arbitrary(unit) => write!(f, "{}-[{}]", self.kind, unit),
        }
    }
}

impl TailwindInstance for TailwindRounded {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let size = match &self.size {
            RoundedSize::None => LengthUnit::px(0.0).get_properties(),
            RoundedSize::Xs => LengthUnit::rem(0.125).get_properties(),
            RoundedSize::Sm | RoundedSize::Default => LengthUnit::rem(0.25).get_properties(),
            RoundedSize::Md => LengthUnit::rem(0.375).get_properties(),
            RoundedSize::Lg => LengthUnit::rem(0.5).get_properties(),
            RoundedSize::Xl => LengthUnit::rem(0.75).get_properties(),
            RoundedSize::Xl2 => LengthUnit::rem(1.0).get_properties(),
            RoundedSize::Xl3 => LengthUnit::rem(1.5).get_properties(),
            RoundedSize::Xl4 => LengthUnit::rem(2.0).get_properties(),
            RoundedSize::Full => LengthUnit::px(9999.0).get_properties(),
            RoundedSize::Arbitrary(unit) => unit.get_properties(),
        };
        match self.kind {
            RoundedKind::Rounded => css_attributes! {
                "border-radius" => &size
            },
            RoundedKind::RoundedT => css_attributes! {
                "border-top-left-radius" => &size,
                "border-top-right-radius" => &size,
            },
            RoundedKind::RoundedR => css_attributes! {
                "border-top-right-radius" => &size,
                "border-bottom-right-radius" => &size,
            },
            RoundedKind::RoundedB => css_attributes! {
                "border-bottom-right-radius" => &size,
                "border-bottom-left-radius" => &size,
            },
            RoundedKind::RoundedL => css_attributes! {
                "border-top-left-radius" => &size,
                "border-bottom-left-radius" => &size,
            },
            RoundedKind::RoundedTL => css_attributes! {
                "border-top-left-radius" => &size,
            },
            RoundedKind::RoundedTR => css_attributes! {
                "border-top-right-radius" => &size,
            },
            RoundedKind::RoundedBL => css_attributes! {
                "border-bottom-left-radius" => &size,
            },
            RoundedKind::RoundedBR => css_attributes! {
                "border-bottom-right-radius" => &size,
            },
            // Logical properties
            RoundedKind::RoundedS => css_attributes! {
                "border-start-start-radius" => &size,
                "border-end-start-radius" => &size,
            },
            RoundedKind::RoundedE => css_attributes! {
                "border-start-end-radius" => &size,
                "border-end-end-radius" => &size,
            },
            RoundedKind::RoundedSS => css_attributes! {
                "border-start-start-radius" => &size,
            },
            RoundedKind::RoundedSE => css_attributes! {
                "border-start-end-radius" => &size,
            },
            RoundedKind::RoundedES => css_attributes! {
                "border-end-start-radius" => &size,
            },
            RoundedKind::RoundedEE => css_attributes! {
                "border-end-end-radius" => &size,
            },
        }
    }
}

impl TailwindRounded {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        match pattern {
            // Physical properties
            ["t" | "8", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedT, arbitrary),
            ["r" | "6", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedR, arbitrary),
            ["b" | "2", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedB, arbitrary),
            ["l" | "4", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedL, arbitrary),
            ["tl" | "7", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedTL, arbitrary),
            ["tr" | "9", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedTR, arbitrary),
            ["bl" | "3", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedBL, arbitrary),
            ["br" | "1", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedBR, arbitrary),
            // Logical properties
            ["s", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedS, arbitrary),
            ["e", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedE, arbitrary),
            ["ss", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedSS, arbitrary),
            ["se", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedSE, arbitrary),
            ["es", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedES, arbitrary),
            ["ee", rest @ ..] => Self::parse_inner(rest, RoundedKind::RoundedEE, arbitrary),
            _ => Self::parse_inner(pattern, RoundedKind::Rounded, arbitrary),
        }
    }
    fn parse_inner(pattern: &[&str], kind: RoundedKind, arbitrary: &TailwindArbitrary) -> Result<Self> {
        if arbitrary.is_some() {
            return Ok(Self { kind, size: RoundedSize::Arbitrary(arbitrary.as_length_or_fraction()?) });
        }
        match pattern {
            ["none"] => Ok(Self { kind, size: RoundedSize::None }),
            ["xs"] => Ok(Self { kind, size: RoundedSize::Xs }),
            ["sm"] => Ok(Self { kind, size: RoundedSize::Sm }),
            ["md"] => Ok(Self { kind, size: RoundedSize::Md }),
            ["lg"] => Ok(Self { kind, size: RoundedSize::Lg }),
            ["xl"] => Ok(Self { kind, size: RoundedSize::Xl }),
            ["2xl"] => Ok(Self { kind, size: RoundedSize::Xl2 }),
            ["3xl"] => Ok(Self { kind, size: RoundedSize::Xl3 }),
            ["4xl"] => Ok(Self { kind, size: RoundedSize::Xl4 }),
            ["full"] => Ok(Self { kind, size: RoundedSize::Full }),
            [] => Ok(Self { kind, size: RoundedSize::Default }),
            _ => syntax_error!(""),
        }
    }
}
