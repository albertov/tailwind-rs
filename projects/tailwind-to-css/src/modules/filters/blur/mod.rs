use super::*;

#[doc=include_str!("readme.md")]
#[derive(Clone, Debug)]
pub struct TailwindBlur {
    px: NumericValue,
    backdrop: Backdrop,
}
impl Display for TailwindBlur {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.backdrop.write(f)?;
        write!(f, "blur-{}", self.px)
    }
}

impl TailwindInstance for TailwindBlur {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        let n = self.px.get_properties(|f| format!("{}px", f));
        // Map keywords to their pixel values
        let blur_value = match n.as_str() {
            "none" => "0".to_string(),
            "sm" => "4px".to_string(),
            "md" => "12px".to_string(),
            "lg" => "16px".to_string(),
            "xl" => "24px".to_string(),
            "2xl" => "40px".to_string(),
            "3xl" => "64px".to_string(),
            _ => n, // For numbers and arbitrary values
        };
        self.backdrop.get_filter_var("blur", format!("blur({})", blur_value))
    }
}

impl TailwindBlur {
    pub fn parse(rest: &[&str], arbitrary: &TailwindArbitrary, backdrop: bool) -> Result<Self> {
        let px = match rest {
            [] if arbitrary.is_none() => 8u32.into(),
            _ => NumericValue::positive_parser("blur", Self::check_valid)(rest, arbitrary)?,
        };
        Ok(Self { px, backdrop: Backdrop::from(backdrop) })
    }

    /// Check if the value is a valid blur keyword
    fn check_valid(value: &str) -> bool {
        matches!(value, "none" | "sm" | "md" | "lg" | "xl" | "2xl" | "3xl")
    }
}
