use crate::{Result, TailwindArbitrary, TailwindBuilder, TailwindInstance, CssAttributes, css_attributes, syntax_error};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub enum GradientDirection {
    ToTop,
    ToTopRight,
    ToRight,
    ToBottomRight,
    ToBottom,
    ToBottomLeft,
    ToLeft,
    ToTopLeft,
}

impl GradientDirection {
    pub fn to_css(&self) -> &'static str {
        match self {
            Self::ToTop => "to top",
            Self::ToTopRight => "to top right",
            Self::ToRight => "to right",
            Self::ToBottomRight => "to bottom right",
            Self::ToBottom => "to bottom",
            Self::ToBottomLeft => "to bottom left",
            Self::ToLeft => "to left",
            Self::ToTopLeft => "to top left",
        }
    }
    
    pub fn parse(pattern: &[&str]) -> Result<Self> {
        match pattern {
            ["t"] => Ok(Self::ToTop),
            ["tr"] => Ok(Self::ToTopRight),
            ["r"] => Ok(Self::ToRight),
            ["br"] => Ok(Self::ToBottomRight),
            ["b"] => Ok(Self::ToBottom),
            ["bl"] => Ok(Self::ToBottomLeft),
            ["l"] => Ok(Self::ToLeft),
            ["tl"] => Ok(Self::ToTopLeft),
            _ => return syntax_error!("Unknown gradient direction: {}", pattern.join("-"))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TailwindBackgroundGradient {
    direction: GradientDirection,
}

impl Display for TailwindBackgroundGradient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            GradientDirection::ToTop => write!(f, "bg-gradient-to-t"),
            GradientDirection::ToTopRight => write!(f, "bg-gradient-to-tr"),
            GradientDirection::ToRight => write!(f, "bg-gradient-to-r"),
            GradientDirection::ToBottomRight => write!(f, "bg-gradient-to-br"),
            GradientDirection::ToBottom => write!(f, "bg-gradient-to-b"),
            GradientDirection::ToBottomLeft => write!(f, "bg-gradient-to-bl"),
            GradientDirection::ToLeft => write!(f, "bg-gradient-to-l"),
            GradientDirection::ToTopLeft => write!(f, "bg-gradient-to-tl"),
        }
    }
}

impl TailwindInstance for TailwindBackgroundGradient {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        css_attributes! {
            "background-image" => format!("linear-gradient({}, var(--tw-gradient-stops))", self.direction.to_css())
        }
    }
}

impl TailwindBackgroundGradient {
    pub fn parse(pattern: &[&str], _arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self {
            direction: GradientDirection::parse(pattern)?,
        })
    }
}