use super::*;

#[derive(Debug, Clone)]
pub struct TailwindStrokeDasharray {
    value: NumericValue,
}

impl Display for TailwindStrokeDasharray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "stroke-dash-{}", self.value)
    }
}

impl TailwindInstance for TailwindStrokeDasharray {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let dasharray = self.value.get_properties(|n| {
            // Map numeric values to appropriate dasharray values
            // stroke-dash-0 => 0 (no dash)
            // stroke-dash-1 through stroke-dash-10 => appropriate dash patterns
            match n {
                n if *n == 0.0 => "0".to_string(),
                n => format!("{}", n),
            }
        });
        css_attributes! {
            "stroke-dasharray" => dasharray
        }
    }
}

impl TailwindStrokeDasharray {
    /// https://tailwindcss.com/docs/stroke-dasharray (conceptual)
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let value = match pattern {
            ["0"] => NumericValue::Number { n: 0.0, negative: false, can_be_negative: false },
            ["1"] => NumericValue::Number { n: 1.0, negative: false, can_be_negative: false },
            ["2"] => NumericValue::Number { n: 2.0, negative: false, can_be_negative: false },
            ["3"] => NumericValue::Number { n: 3.0, negative: false, can_be_negative: false },
            ["4"] => NumericValue::Number { n: 4.0, negative: false, can_be_negative: false },
            ["5"] => NumericValue::Number { n: 5.0, negative: false, can_be_negative: false },
            ["6"] => NumericValue::Number { n: 6.0, negative: false, can_be_negative: false },
            ["7"] => NumericValue::Number { n: 7.0, negative: false, can_be_negative: false },
            ["8"] => NumericValue::Number { n: 8.0, negative: false, can_be_negative: false },
            ["9"] => NumericValue::Number { n: 9.0, negative: false, can_be_negative: false },
            ["10"] => NumericValue::Number { n: 10.0, negative: false, can_be_negative: false },
            [] if arbitrary.is_some() => NumericValue::Arbitrary(TailwindArbitrary::new(arbitrary)?),
            _ => return Err(TailwindError::syntax_error("Unknown stroke-dasharray value")),
        };
        Ok(Self { value })
    }
}