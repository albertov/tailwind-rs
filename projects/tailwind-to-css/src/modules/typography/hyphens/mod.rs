use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindHyphens {
    kind: HyphensKind,
}

#[derive(Debug, Clone)]
enum HyphensKind {
    None,
    Manual,
    Auto,
    Arbitrary(TailwindArbitrary),
}

impl Display for TailwindHyphens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            HyphensKind::None => write!(f, "hyphens-none"),
            HyphensKind::Manual => write!(f, "hyphens-manual"),
            HyphensKind::Auto => write!(f, "hyphens-auto"),
            HyphensKind::Arbitrary(s) => write!(f, "hyphens-{}", s.get_class()),
        }
    }
}

impl TailwindInstance for TailwindHyphens {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        match &self.kind {
            HyphensKind::None => css_attributes! {
                "hyphens" => "none"
            },
            HyphensKind::Manual => css_attributes! {
                "hyphens" => "manual"
            },
            HyphensKind::Auto => css_attributes! {
                "hyphens" => "auto"
            },
            HyphensKind::Arbitrary(s) => css_attributes! {
                "hyphens" => s.get_properties()
            },
        }
    }
}

impl TailwindHyphens {
    /// Create a hyphens instance from a string value
    pub fn from(value: &str) -> Self {
        let kind = match value {
            "none" => HyphensKind::None,
            "manual" => HyphensKind::Manual,
            "auto" => HyphensKind::Auto,
            _ => HyphensKind::None, // Default to none for unknown values
        };
        Self { kind }
    }

    /// Parse hyphens utilities
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        let kind = match pattern {
            [] => HyphensKind::Arbitrary(arbitrary.to_owned()),
            ["none"] => HyphensKind::None,
            ["manual"] => HyphensKind::Manual,
            ["auto"] => HyphensKind::Auto,
            _ => return syntax_error!("Unknown hyphens pattern: {}", pattern.join("-")),
        };
        Ok(Self { kind })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hyphens_display() {
        assert_eq!(TailwindHyphens::from("none").to_string(), "hyphens-none");
        assert_eq!(TailwindHyphens::from("manual").to_string(), "hyphens-manual");
        assert_eq!(TailwindHyphens::from("auto").to_string(), "hyphens-auto");
    }
    
    #[test]
    fn test_hyphens_attributes() {
        let builder = TailwindBuilder::default();
        
        // Test hyphens-none
        let none = TailwindHyphens::from("none");
        let attrs = none.attributes(&builder);
        assert!(attrs.to_string().contains("hyphens:none"));
        
        // Test hyphens-manual
        let manual = TailwindHyphens::from("manual");
        let attrs = manual.attributes(&builder);
        assert!(attrs.to_string().contains("hyphens:manual"));
        
        // Test hyphens-auto
        let auto = TailwindHyphens::from("auto");
        let attrs = auto.attributes(&builder);
        assert!(attrs.to_string().contains("hyphens:auto"));
    }
    
    #[test]
    fn test_hyphens_parse() {
        let arbitrary = TailwindArbitrary::from("inherit");
        
        // Test parsing with pattern
        let none = TailwindHyphens::parse(&["none"], &arbitrary).unwrap();
        assert_eq!(none.to_string(), "hyphens-none");
        
        let manual = TailwindHyphens::parse(&["manual"], &arbitrary).unwrap();
        assert_eq!(manual.to_string(), "hyphens-manual");
        
        let auto = TailwindHyphens::parse(&["auto"], &arbitrary).unwrap();
        assert_eq!(auto.to_string(), "hyphens-auto");
        
        // Test arbitrary value
        let arb = TailwindHyphens::parse(&[], &arbitrary).unwrap();
        assert_eq!(arb.to_string(), "hyphens-[inherit]");
    }
    
    #[test]
    fn test_hyphens_unknown_pattern() {
        let arbitrary = TailwindArbitrary::from("test");
        let result = TailwindHyphens::parse(&["unknown"], &arbitrary);
        assert!(result.is_err());
    }
}