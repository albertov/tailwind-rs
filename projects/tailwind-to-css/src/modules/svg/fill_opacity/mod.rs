use super::*;

#[derive(Debug, Clone)]
pub struct TailwindFillOpacity {
    percent: NumericValue,
}

impl Display for TailwindFillOpacity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fill-opacity-{}", self.percent)
    }
}

impl TailwindInstance for TailwindFillOpacity {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let opacity = self.percent.get_properties(|n| {
            // Convert percentage to decimal (0-100 -> 0.0-1.0)
            format!("{}", n / 100.0)
        });
        css_attributes! {
            "fill-opacity" => opacity
        }
    }
}

impl TailwindFillOpacity {
    /// https://tailwindcss.com/docs/fill-opacity (conceptual)
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        // First try to parse as a known opacity value
        let percent = match pattern {
            ["0"] => NumericValue::Number { n: 0.0, negative: false, can_be_negative: false },
            ["5"] => NumericValue::Number { n: 5.0, negative: false, can_be_negative: false },
            ["10"] => NumericValue::Number { n: 10.0, negative: false, can_be_negative: false },
            ["20"] => NumericValue::Number { n: 20.0, negative: false, can_be_negative: false },
            ["25"] => NumericValue::Number { n: 25.0, negative: false, can_be_negative: false },
            ["30"] => NumericValue::Number { n: 30.0, negative: false, can_be_negative: false },
            ["40"] => NumericValue::Number { n: 40.0, negative: false, can_be_negative: false },
            ["50"] => NumericValue::Number { n: 50.0, negative: false, can_be_negative: false },
            ["60"] => NumericValue::Number { n: 60.0, negative: false, can_be_negative: false },
            ["70"] => NumericValue::Number { n: 70.0, negative: false, can_be_negative: false },
            ["75"] => NumericValue::Number { n: 75.0, negative: false, can_be_negative: false },
            ["80"] => NumericValue::Number { n: 80.0, negative: false, can_be_negative: false },
            ["90"] => NumericValue::Number { n: 90.0, negative: false, can_be_negative: false },
            ["95"] => NumericValue::Number { n: 95.0, negative: false, can_be_negative: false },
            ["100"] => NumericValue::Number { n: 100.0, negative: false, can_be_negative: false },
            [] if arbitrary.is_some() => NumericValue::Arbitrary(TailwindArbitrary::new(arbitrary)?),
            _ => return Err(TailwindError::syntax_error("Unknown fill-opacity value")),
        };
        Ok(Self { percent })
    }
}