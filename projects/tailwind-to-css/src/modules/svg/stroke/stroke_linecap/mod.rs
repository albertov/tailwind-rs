use super::*;

#[derive(Debug, Clone)]
pub struct TailwindStrokeLinecap {
    value: String,
}

impl Display for TailwindStrokeLinecap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stroke-cap-{}", self.value)
    }
}

impl TailwindInstance for TailwindStrokeLinecap {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let linecap = match self.value.as_str() {
            "butt" => "butt",
            "round" => "round",
            "square" => "square",
            _ => "butt", // default
        };
        css_attributes! {
            "stroke-linecap" => linecap
        }
    }
}

impl TailwindStrokeLinecap {
    /// https://tailwindcss.com/docs/stroke-linecap (conceptual)
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let value = match pattern {
            ["butt"] => "butt".to_string(),
            ["round"] => "round".to_string(),
            ["square"] => "square".to_string(),
            [] if arbitrary.is_some() => arbitrary.get_properties(),
            _ => return Err(TailwindError::syntax_error("Unknown stroke-linecap value")),
        };
        Ok(Self { value })
    }
}