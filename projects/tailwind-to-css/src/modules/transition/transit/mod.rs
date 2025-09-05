use super::*;

#[derive(Clone, Debug)]
enum Transition {
    None,
    All,
    Default,
    Colors,
    Filter,
    Opacity,
    Shadow,
    Transform,
    Arbitrary(TailwindArbitrary),
}

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindTransition {
    kind: Transition,
}

impl Display for Transition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "-none"),
            Self::All => write!(f, "-all"),
            Self::Default => write!(f, ""),
            Self::Colors => write!(f, "-colors"),
            Self::Filter => write!(f, "-filter"),
            Self::Opacity => write!(f, "-opacity"),
            Self::Shadow => write!(f, "-shadow"),
            Self::Transform => write!(f, "-transform"),
            Self::Arbitrary(g) => write!(f, "-[{}]", g.get_properties()),
        }
    }
}

impl Display for TailwindTransition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "transition{}", self.kind)
    }
}

impl TailwindInstance for TailwindTransition {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let (property, timing, duration, delay) = match &self.kind {
            Transition::None => ("none", "", "", ""),
            Transition::All => ("all", "cubic-bezier(0.4, 0, 0.2, 1)", "150ms", ""),
            Transition::Default => (
                "color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter",
                "cubic-bezier(0.4, 0, 0.2, 1)",
                "150ms",
                ""
            ),
            Transition::Colors => (
                "color, background-color, border-color, text-decoration-color, fill, stroke",
                "cubic-bezier(0.4, 0, 0.2, 1)",
                "150ms",
                ""
            ),
            Transition::Filter => ("filter", "cubic-bezier(0.4, 0, 0.2, 1)", "150ms", ""),
            Transition::Opacity => ("opacity", "cubic-bezier(0.4, 0, 0.2, 1)", "150ms", ""),
            Transition::Shadow => ("box-shadow", "cubic-bezier(0.4, 0, 0.2, 1)", "150ms", ""),
            Transition::Transform => ("transform", "cubic-bezier(0.4, 0, 0.2, 1)", "150ms", ""),
            Transition::Arbitrary(s) => {
                return css_attributes! {
                    "transition-property" => s.get_properties()
                }
            }
        };
        
        let mut attrs = css_attributes! {
            "transition-property" => property
        };
        
        if !timing.is_empty() {
            attrs.insert("transition-timing-function", timing);
        }
        if !duration.is_empty() {
            attrs.insert("transition-duration", duration);
        }
        if !delay.is_empty() {
            attrs.insert("transition-delay", delay);
        }
        
        attrs
    }
}

impl TailwindTransition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Transition::parse(pattern, arbitrary)? })
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: Transition::parse_arbitrary(arbitrary)? })
    }
}

impl Transition {
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let t = match pattern {
            [] if arbitrary.is_none() => Self::Default,
            [] => Self::parse_arbitrary(arbitrary)?,
            ["none"] => Self::None,
            ["all"] => Self::All,
            ["colors"] => Self::Colors,
            ["filter"] => Self::Filter,
            ["opacity"] => Self::Opacity,
            ["shadow"] => Self::Shadow,
            ["transform"] => Self::Transform,
            _ => return syntax_error!("Unknown transition instructions: {}", pattern.join("-")),
        };
        Ok(t)
    }
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self::Arbitrary(TailwindArbitrary::new(arbitrary)?))
    }
}
