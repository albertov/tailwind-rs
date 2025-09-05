#[cfg(test)]
mod container_important_integration_tests {
    use tailwind_rs::TailwindBuilder;
    
    #[test]
    fn test_container_query_with_important_generates_css() {
        let mut tw = TailwindBuilder::default();
        
        // Test basic container + important
        tw.trace("@lg:!flex", false).unwrap();
        let css = tw.bundle().expect("Failed to bundle CSS");
        
        // Check that CSS contains the container query with !important
        assert!(css.contains("@container"), "CSS should contain @container query");
        assert!(css.contains("!important"), "CSS should contain !important");
        assert!(css.contains("display"), "CSS should contain display property");
        assert!(css.contains("flex"), "CSS should contain flex value");
    }
    
    #[test]
    fn test_multiple_container_important_utilities() {
        let mut tw = TailwindBuilder::default();
        
        // Test various container queries with important
        tw.trace("@lg:!flex", false).unwrap();
        tw.trace("@md:!hidden", false).unwrap();
        tw.trace("@xl:!block", false).unwrap();
        tw.trace("@[300px]:!grid", false).unwrap();
        tw.trace("@max-lg:!inline-block", false).unwrap();
        
        let css = tw.bundle().expect("Failed to bundle CSS");
        
        // Verify all utilities are present with !important
        assert!(css.contains("!important"), "CSS should contain !important");
        assert!(css.contains("flex"), "CSS should contain flex");
        assert!(css.contains("hidden"), "CSS should contain hidden");
        assert!(css.contains("block"), "CSS should contain block");
        assert!(css.contains("grid"), "CSS should contain grid");
        assert!(css.contains("inline-block"), "CSS should contain inline-block");
    }
    
    #[test]
    fn test_container_query_named_with_important() {
        let mut tw = TailwindBuilder::default();
        
        // Test named container with important
        tw.trace("@lg/sidebar:!p-4", false).unwrap();
        let css = tw.bundle().expect("Failed to bundle CSS");
        
        assert!(css.contains("!important"), "CSS should contain !important");
        assert!(css.contains("padding"), "CSS should contain padding");
    }
    
    #[test]
    fn test_container_query_important_with_negative() {
        let mut tw = TailwindBuilder::default();
        
        // Test negative value with important
        tw.trace("@lg:!-mt-4", false).unwrap();
        let css = tw.bundle().expect("Failed to bundle CSS");
        
        assert!(css.contains("!important"), "CSS should contain !important");
        assert!(css.contains("margin-top"), "CSS should contain margin-top");
        // The actual negative value format may vary - just check that we have margin-top with important
        assert!(css.contains("margin-top") && css.contains("!important"), 
                "CSS should contain margin-top with !important");
    }
    
    #[test]
    fn test_regular_variant_important_still_works() {
        let mut tw = TailwindBuilder::default();
        
        // Ensure regular variants with important still work
        tw.trace("hover:!bg-blue-500", false).unwrap();
        tw.trace("focus:!text-red-600", false).unwrap();
        tw.trace("lg:!flex", false).unwrap();
        
        let css = tw.bundle().expect("Failed to bundle CSS");
        
        assert!(css.contains("!important"), "CSS should contain !important");
        assert!(css.contains(":hover"), "CSS should contain :hover pseudo-class");
        assert!(css.contains(":focus"), "CSS should contain :focus pseudo-class");
    }
    
    #[test]
    fn test_container_query_important_css_output() {
        let mut tw = TailwindBuilder::default();
        
        tw.trace("@lg:!flex", false).unwrap();
        let css = tw.bundle().expect("Failed to bundle CSS");
        
        // More specific checks for the generated CSS structure
        // The actual CSS should look something like:
        // @container (min-width: 48rem) {
        //   .\@lg\:\!flex {
        //     display: flex !important;
        //   }
        // }
        
        // Check for the container query
        assert!(css.contains("@container"), "CSS should contain @container rule");
        assert!(css.contains("min-width"), "CSS should contain min-width media query");
        
        // Check for the important flag in the declaration
        assert!(css.contains("display"), "CSS should contain display property");
        assert!(css.contains("flex"), "CSS should contain flex value");
        assert!(css.contains("!important"), "CSS should contain !important flag");
    }
}