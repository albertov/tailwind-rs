use super::*;

#[derive(Debug, Clone)]
pub struct TailwindStrokeLinejoin {
    value: String,
}

impl Display for TailwindStrokeLinejoin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stroke-join-{}", self.value)
    }
}

impl TailwindInstance for TailwindStrokeLinejoin {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let linejoin = match self.value.as_str() {
            "miter" => "miter",
            "round" => "round",
            "bevel" => "bevel",
            _ => "miter", // default
        };
        css_attributes! {
            "stroke-linejoin" => linejoin
        }
    }
}

impl TailwindStrokeLinejoin {
    /// https://tailwindcss.com/docs/stroke-linejoin (conceptual)
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let value = match pattern {
            ["miter"] => "miter".to_string(),
            ["round"] => "round".to_string(),
            ["bevel"] => "bevel".to_string(),
            [] if arbitrary.is_some() => arbitrary.get_properties(),
            _ => return Err(TailwindError::syntax_error("Unknown stroke-linejoin value")),
        };
        Ok(Self { value })
    }
}