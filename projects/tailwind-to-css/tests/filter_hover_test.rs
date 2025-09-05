#[cfg(test)]
mod tests {
    use tailwind_css::TailwindBuilder;

    #[test]
    fn test_filter_hover_with_variables() {
        // Test that filter utilities set their specific CSS variables
        // Note: Variable initialization is now handled by preflight
        let mut tw = TailwindBuilder::default();
        
        // Test brightness-110
        let result = tw.inline("brightness-110");
        assert!(result.is_ok(), "Failed to parse brightness-110");
        let (_classes, css) = result.unwrap();
        
        println!("brightness-110 CSS:\n{}", css);
        
        // Should only set the specific brightness variable
        assert!(css.contains("--tw-brightness:brightness(1.1)"), "Missing --tw-brightness value");
        // Should include the filter property with all variables (composed by preflight)
        assert!(css.contains("filter:var(--tw-blur) var(--tw-brightness)"), "Missing filter property");
        
        // Test hover:brightness-125
        let result = tw.inline("hover:brightness-125");
        assert!(result.is_ok(), "Failed to parse hover:brightness-125");
        let (_classes, css) = result.unwrap();
        
        println!("hover:brightness-125 CSS:\n{}", css);
        
        // Hover state should only set the specific variable
        assert!(css.contains("--tw-brightness:brightness(1.25)"), "Missing hover brightness value");
        assert!(css.contains("filter:var(--tw-blur) var(--tw-brightness)"), "Missing filter property in hover");
    }

    #[test]
    fn test_group_hover_filter() {
        // Test group hover with filter effects
        let mut tw = TailwindBuilder::default();
        
        // Test group-hover:brightness-150
        let result = tw.inline("group-hover:brightness-150");
        assert!(result.is_ok(), "Failed to parse group-hover:brightness-150");
        let (_classes, css) = result.unwrap();
        
        println!("group-hover:brightness-150 CSS:\n{}", css);
        
        // The group-hover variant should only set the specific variable
        assert!(css.contains("--tw-brightness:brightness(1.5)"), "Should contain brightness value");
        
        // Should include the filter property
        assert!(css.contains("filter:var(--tw-blur) var(--tw-brightness)"), "Missing filter property in group-hover");
    }

    #[test]
    fn test_multiple_filter_utilities() {
        // Test combining multiple filter utilities
        let mut tw = TailwindBuilder::default();
        
        // Test saturate-150
        let result = tw.inline("saturate-150");
        assert!(result.is_ok(), "Failed to parse saturate-150");
        let (_classes, css) = result.unwrap();
        
        println!("saturate-150 CSS:\n{}", css);
        
        // Each utility should only set its specific variable
        assert!(css.contains("--tw-saturate:saturate(1.5)"));
        assert!(css.contains("filter:var(--tw-blur) var(--tw-brightness)"), "Missing filter property");
        
        // Test grayscale-50
        let result = tw.inline("grayscale-50");
        assert!(result.is_ok(), "Failed to parse grayscale-50");
        let (_classes, css) = result.unwrap();
        
        println!("grayscale-50 CSS:\n{}", css);
        assert!(css.contains("--tw-grayscale:grayscale(0.5)"));
        assert!(css.contains("filter:var(--tw-blur) var(--tw-brightness)"), "Missing filter property");
    }

    #[test]
    fn test_backdrop_filter_variables() {
        // Test backdrop filter utilities
        let mut tw = TailwindBuilder::default();
        
        // Test backdrop-brightness-110
        let result = tw.inline("backdrop-brightness-110");
        assert!(result.is_ok(), "Failed to parse backdrop-brightness-110");
        let (_classes, css) = result.unwrap();
        
        println!("backdrop-brightness-110 CSS:\n{}", css);
        
        // Should only set the specific backdrop brightness variable
        assert!(css.contains("--tw-backdrop-brightness:brightness(1.1)"));
        // Should include the backdrop-filter property
        assert!(css.contains("backdrop-filter:var(--tw-backdrop-blur) var(--tw-backdrop-brightness)"), "Missing backdrop-filter property");
        
        // Test hover:backdrop-brightness-125
        let result = tw.inline("hover:backdrop-brightness-125");
        assert!(result.is_ok(), "Failed to parse hover:backdrop-brightness-125");
        let (_classes, css) = result.unwrap();
        
        println!("hover:backdrop-brightness-125 CSS:\n{}", css);
        assert!(css.contains("--tw-backdrop-brightness:brightness(1.25)"));
        assert!(css.contains("backdrop-filter:var(--tw-backdrop-blur) var(--tw-backdrop-brightness)"), "Missing backdrop-filter property in hover");
    }

    #[test]
    fn test_real_world_hover_scenario() {
        // Test a real-world scenario where we have both base and hover states
        let mut tw = TailwindBuilder::default();
        
        // Test the combination that would typically fail
        let result = tw.inline("brightness-110 group-hover:brightness-125");
        assert!(result.is_ok(), "Failed to parse combined utilities");
        let (_classes, css) = result.unwrap();
        
        println!("Combined CSS:\n{}", css);
        
        // Note: when using inline() with multiple classes, only the last one's CSS is returned
        // The group-hover variant should set its specific brightness value
        assert!(css.contains("--tw-brightness:brightness(1.25)"), "Should contain the group-hover brightness value");
        
        // Should include the filter property
        assert!(css.contains("filter:var(--tw-blur) var(--tw-brightness)"), "Should have filter property");
    }
}