use super::*;
use crate::syntax_error;
use std::collections::BTreeSet;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBorderSpacing {
    axis: BorderSpacingAxis,
    size: SpacingSize,
}

#[derive(Clone, Debug)]
enum BorderSpacingAxis {
    All,
    X,
    Y,
}

#[derive(Debug, Clone)]
enum SpacingSize {
    Unit(f32),
    Standard(String),
    Arbitrary(TailwindArbitrary),
}

impl Display for SpacingSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unit(s) => write!(f, "{}", s),
            Self::Standard(s) => write!(f, "{}", s),
            Self::Arbitrary(s) => s.write(f),
        }
    }
}

impl SpacingSize {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary, check_valid: &'static impl Fn(&str) -> bool) -> Result<Self> {
        match pattern {
            [] => Self::parse_arbitrary(arbitrary),
            ["px"] => Ok(Self::Arbitrary(TailwindArbitrary::from("1px"))),
            [n] if check_valid(n) => Ok(Self::Standard(n.to_string())),
            [n] => Ok(Self::Unit(TailwindArbitrary::from(*n).as_float()?)),
            _ => syntax_error!("Unknown border-spacing instructions: {}", pattern.join("-")),
        }
    }

    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }

    pub fn get_properties(&self) -> String {
        match self {
            Self::Unit(x) => {
                if *x == 0.0 {
                    "0px".to_string()
                } else {
                    format!("{}rem", x / 4.0)
                }
            },
            Self::Standard(x) => x.to_string(),
            Self::Arbitrary(x) => x.get_properties(),
        }
    }
}

impl Display for TailwindBorderSpacing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "border-spacing")?;
        match &self.axis {
            BorderSpacingAxis::All => {},
            BorderSpacingAxis::X => write!(f, "-x")?,
            BorderSpacingAxis::Y => write!(f, "-y")?,
        }
        // Handle arbitrary values which already include brackets
        match &self.size {
            SpacingSize::Arbitrary(arb) => arb.write(f),
            _ => write!(f, "-{}", self.size),
        }
    }
}

impl TailwindBorderSpacing {
    /// <https://tailwindcss.com/docs/border-spacing>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (axis, rest) = match pattern {
            ["x", rest @ ..] => (BorderSpacingAxis::X, rest),
            ["y", rest @ ..] => (BorderSpacingAxis::Y, rest),
            rest => (BorderSpacingAxis::All, rest),
        };
        
        let size = SpacingSize::parse(rest, arbitrary, &Self::check_valid)?;
        Ok(Self { axis, size })
    }

    /// dispatch to [border-spacing](https://developer.mozilla.org/en-US/docs/Web/CSS/border-spacing)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            axis: BorderSpacingAxis::All,
            size: SpacingSize::parse_arbitrary(arbitrary)?,
        })
    }

    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-spacing#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}

impl TailwindInstance for TailwindBorderSpacing {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        match &self.axis {
            BorderSpacingAxis::All => css_attributes! {
                "border-spacing" => self.size.get_properties()
            },
            BorderSpacingAxis::X => css_attributes! {
                "--tw-border-spacing-x" => self.size.get_properties(),
                "border-spacing" => "var(--tw-border-spacing-x) var(--tw-border-spacing-y)"
            },
            BorderSpacingAxis::Y => css_attributes! {
                "--tw-border-spacing-y" => self.size.get_properties(),
                "border-spacing" => "var(--tw-border-spacing-x) var(--tw-border-spacing-y)"
            },
        }
    }
}