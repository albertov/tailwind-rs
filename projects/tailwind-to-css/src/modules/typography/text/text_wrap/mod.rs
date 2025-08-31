use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindTextWrap {
    kind: TextWrapKind,
}

#[derive(Debug, Clone)]
enum TextWrapKind {
    Wrap,
    NoWrap,
    Balance,
    Pretty,
    Arbitrary(TailwindArbitrary),
}

impl Display for TailwindTextWrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TextWrapKind::Wrap => write!(f, "text-wrap"),
            TextWrapKind::NoWrap => write!(f, "text-nowrap"),
            TextWrapKind::Balance => write!(f, "text-balance"),
            TextWrapKind::Pretty => write!(f, "text-pretty"),
            TextWrapKind::Arbitrary(s) => write!(f, "text-wrap-{}", s.get_class()),
        }
    }
}

impl TailwindInstance for TailwindTextWrap {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        match &self.kind {
            TextWrapKind::Wrap => css_attributes! {
                "text-wrap" => "wrap"
            },
            TextWrapKind::NoWrap => css_attributes! {
                "text-wrap" => "nowrap"
            },
            TextWrapKind::Balance => css_attributes! {
                "text-wrap" => "balance"
            },
            TextWrapKind::Pretty => css_attributes! {
                "text-wrap" => "pretty"
            },
            TextWrapKind::Arbitrary(s) => css_attributes! {
                "text-wrap" => s.get_properties()
            },
        }
    }
}

impl TailwindTextWrap {
    /// Create a text-wrap instance
    pub fn from(value: &str) -> Self {
        let kind = match value {
            "wrap" => TextWrapKind::Wrap,
            "nowrap" => TextWrapKind::NoWrap,
            "balance" => TextWrapKind::Balance,
            "pretty" => TextWrapKind::Pretty,
            _ => TextWrapKind::Wrap,
        };
        Self { kind }
    }

    /// Parse text-wrap utilities
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => TextWrapKind::Arbitrary(arbitrary.to_owned()),
            ["wrap"] => TextWrapKind::Wrap,
            ["nowrap"] => TextWrapKind::NoWrap,
            ["balance"] => TextWrapKind::Balance,
            ["pretty"] => TextWrapKind::Pretty,
            _ => return syntax_error!("Unknown text-wrap pattern: {}", pattern.join("-")),
        };
        Ok(Self { kind })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_wrap_display() {
        assert_eq!(TailwindTextWrap::from("wrap").to_string(), "text-wrap");
        assert_eq!(TailwindTextWrap::from("nowrap").to_string(), "text-nowrap");
        assert_eq!(TailwindTextWrap::from("balance").to_string(), "text-balance");
        assert_eq!(TailwindTextWrap::from("pretty").to_string(), "text-pretty");
    }
    
    #[test]
    fn test_text_wrap_attributes() {
        let builder = TailwindBuilder::default();
        
        // Test text-wrap
        let wrap = TailwindTextWrap::from("wrap");
        let attrs = wrap.attributes(&builder);
        assert!(attrs.to_string().contains("text-wrap:wrap"));
        
        // Test text-nowrap
        let nowrap = TailwindTextWrap::from("nowrap");
        let attrs = nowrap.attributes(&builder);
        assert!(attrs.to_string().contains("text-wrap:nowrap"));
        
        // Test text-balance
        let balance = TailwindTextWrap::from("balance");
        let attrs = balance.attributes(&builder);
        assert!(attrs.to_string().contains("text-wrap:balance"));
        
        // Test text-pretty
        let pretty = TailwindTextWrap::from("pretty");
        let attrs = pretty.attributes(&builder);
        assert!(attrs.to_string().contains("text-wrap:pretty"));
    }
}