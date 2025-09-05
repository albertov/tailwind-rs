use super::*;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindCaptionSide {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindCaptionSide => "caption-side");

impl Display for TailwindCaptionSide {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            StandardValue::Keyword(s) => match s.as_str() {
                "top" | "bottom" => write!(f, "caption-{}", s),
                _ => write!(f, "caption-side-{}", s),
            },
            StandardValue::Arbitrary(s) => s.write_class(f, "caption"),
        }
    }
}

impl TailwindCaptionSide {
    /// <https://tailwindcss.com/docs/caption-side>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parser("caption-side", &Self::check_valid)(pattern, arbitrary)? })
    }
    
    /// dispatch to [caption-side](https://developer.mozilla.org/en-US/docs/Web/CSS/caption-side)
    pub fn parse_arbitrary(arbitrary: &TailwindArbitrary) -> Result<Self> {
        Ok(Self { kind: StandardValue::parse_arbitrary(arbitrary)? })
    }
    
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/caption-side#syntax>
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec!["top", "bottom", "inherit", "initial", "revert", "unset"]);
        set.contains(mode)
    }
}