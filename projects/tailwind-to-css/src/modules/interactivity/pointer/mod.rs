use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindPointerEvents {
    kind: StandardValue,
}

crate::macros::sealed::keyword_instance!(TailwindPointerEvents => "pointer-events");

impl Display for TailwindPointerEvents {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "pointer-events-{}", self.kind)
    }
}

impl TailwindPointerEvents {
    /// <https://tailwindcss.com/docs/pointer-events>
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = StandardValue::parser("pointer-events", &Self::check_valid)(pattern, arbitrary)?;
        Ok(Self { kind })
    }
    /// <https://tailwindcss.com/docs/pointer-events>
    /// Tailwind CSS only supports 'none' and 'auto' for pointer-events
    pub fn check_valid(mode: &str) -> bool {
        matches!(mode, "none" | "auto")
    }
}
