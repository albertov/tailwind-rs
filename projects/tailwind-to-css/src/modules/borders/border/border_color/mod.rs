use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBorderColor {
    kind: BorderColorKind,
    color: TailwindColor,
}

#[derive(Copy, Clone, Debug)]
enum BorderColorKind {
    Border,
    BorderX,
    BorderY,
    BorderT,
    BorderR,
    BorderB,
    BorderL,
    BorderS,
    BorderE,
}

impl Display for BorderColorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Border => write!(f, "border"),
            Self::BorderX => write!(f, "border-x"),
            Self::BorderY => write!(f, "border-y"),
            Self::BorderT => write!(f, "border-t"),
            Self::BorderR => write!(f, "border-r"),
            Self::BorderB => write!(f, "border-b"),
            Self::BorderL => write!(f, "border-l"),
            Self::BorderS => write!(f, "border-s"),
            Self::BorderE => write!(f, "border-e"),
        }
    }
}

impl Display for TailwindBorderColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.kind, self.color)
    }
}

impl TailwindInstance for TailwindBorderColor {
    fn attributes(&self, ctx: &TailwindBuilder) -> CssAttributes {
        let color_value = self.color.get_properties(ctx);
        match self.kind {
            BorderColorKind::Border => css_attributes! {
                "border-color" => color_value
            },
            BorderColorKind::BorderX => css_attributes! {
                "border-left-color" => &color_value,
                "border-right-color" => color_value,
            },
            BorderColorKind::BorderY => css_attributes! {
                "border-top-color" => &color_value,
                "border-bottom-color" => color_value,
            },
            BorderColorKind::BorderT => css_attributes! {
                "border-top-color" => color_value,
            },
            BorderColorKind::BorderR => css_attributes! {
                "border-right-color" => color_value,
            },
            BorderColorKind::BorderB => css_attributes! {
                "border-bottom-color" => color_value,
            },
            BorderColorKind::BorderL => css_attributes! {
                "border-left-color" => color_value,
            },
            BorderColorKind::BorderS => css_attributes! {
                "border-inline-start-color" => color_value,
            },
            BorderColorKind::BorderE => css_attributes! {
                "border-inline-end-color" => color_value,
            },
        }
    }
}

impl<T> From<T> for TailwindBorderColor
where
    T: Into<TailwindColor>,
{
    fn from(color: T) -> Self {
        Self { 
            kind: BorderColorKind::Border,
            color: color.into() 
        }
    }
}

impl TailwindBorderColor {
    ///
    pub fn parse(input: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let (kind, rest) = match input {
            ["t", rest @ ..] => (BorderColorKind::BorderT, rest),
            ["r", rest @ ..] => (BorderColorKind::BorderR, rest),
            ["b", rest @ ..] => (BorderColorKind::BorderB, rest),
            ["l", rest @ ..] => (BorderColorKind::BorderL, rest),
            ["x", rest @ ..] => (BorderColorKind::BorderX, rest),
            ["y", rest @ ..] => (BorderColorKind::BorderY, rest),
            ["s", rest @ ..] => (BorderColorKind::BorderS, rest),
            ["e", rest @ ..] => (BorderColorKind::BorderE, rest),
            _ => (BorderColorKind::Border, input),
        };
        
        Ok(Self { 
            kind,
            color: TailwindColor::parse(rest, arbitrary)? 
        })
    }
    
    ///
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { 
            kind: BorderColorKind::Border,
            color: TailwindColor::parse_arbitrary(arbitrary)? 
        })
    }
    
    /// Parse with optional opacity modifier
    pub fn parse_with_opacity(input: &[&str], arbitrary: &TailwindArbitrary, opacity: Option<&str>) -> Result<Self> {
        let (kind, rest) = match input {
            ["t", rest @ ..] => (BorderColorKind::BorderT, rest),
            ["r", rest @ ..] => (BorderColorKind::BorderR, rest),
            ["b", rest @ ..] => (BorderColorKind::BorderB, rest),
            ["l", rest @ ..] => (BorderColorKind::BorderL, rest),
            ["x", rest @ ..] => (BorderColorKind::BorderX, rest),
            ["y", rest @ ..] => (BorderColorKind::BorderY, rest),
            ["s", rest @ ..] => (BorderColorKind::BorderS, rest),
            ["e", rest @ ..] => (BorderColorKind::BorderE, rest),
            _ => (BorderColorKind::Border, input),
        };
        
        Ok(Self { 
            kind,
            color: TailwindColor::parse_with_opacity(rest, arbitrary, opacity)? 
        })
    }
}