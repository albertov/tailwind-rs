#[cfg(test)]
mod typography_utilities_tests {
    use tailwind_css::TailwindBuilder;

    #[test]
    fn test_line_clamp_utilities() {
        // Test line-clamp-1
        let mut builder = TailwindBuilder::default();
        builder.trace("line-clamp-1", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("overflow:hidden"));
        assert!(css.contains("display:-webkit-box"));
        assert!(css.contains("-webkit-box-orient:vertical"));
        assert!(css.contains("-webkit-line-clamp:1"));

        // Test line-clamp-3
        let mut builder = TailwindBuilder::default();
        builder.trace("line-clamp-3", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("-webkit-line-clamp:3"));

        // Test line-clamp-6
        let mut builder = TailwindBuilder::default();
        builder.trace("line-clamp-6", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("-webkit-line-clamp:6"));

        // Test line-clamp-none
        let mut builder = TailwindBuilder::default();
        builder.trace("line-clamp-none", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("overflow:visible"));
        assert!(css.contains("display:block"));
        assert!(css.contains("-webkit-line-clamp:none"));
    }

    #[test]
    fn test_line_clamp_arbitrary_values() {
        // Test arbitrary line-clamp value
        let mut builder = TailwindBuilder::default();
        builder.trace("line-clamp-[10]", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("-webkit-line-clamp:10"));
    }

    #[test]
    fn test_hyphens_utilities() {
        // Test hyphens-none
        let mut builder = TailwindBuilder::default();
        builder.trace("hyphens-none", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("hyphens:none"));

        // Test hyphens-manual
        let mut builder = TailwindBuilder::default();
        builder.trace("hyphens-manual", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("hyphens:manual"));

        // Test hyphens-auto
        let mut builder = TailwindBuilder::default();
        builder.trace("hyphens-auto", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("hyphens:auto"));
    }

    #[test]
    fn test_hyphens_arbitrary_values() {
        // Test arbitrary hyphens value
        let mut builder = TailwindBuilder::default();
        builder.trace("hyphens-[inherit]", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("hyphens:inherit"));
    }

    #[test]
    fn test_line_clamp_with_modifiers() {
        // Test with responsive modifier
        let mut builder = TailwindBuilder::default();
        builder.trace("sm:line-clamp-2", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("@media"));
        assert!(css.contains("-webkit-line-clamp:2"));

        // Test with hover modifier
        let mut builder = TailwindBuilder::default();
        builder.trace("hover:line-clamp-none", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains(":hover"));
        assert!(css.contains("-webkit-line-clamp:none"));
    }

    #[test]
    fn test_hyphens_with_modifiers() {
        // Test with responsive modifier
        let mut builder = TailwindBuilder::default();
        builder.trace("lg:hyphens-auto", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains("@media"));
        assert!(css.contains("hyphens:auto"));

        // Test with focus modifier
        let mut builder = TailwindBuilder::default();
        builder.trace("focus:hyphens-none", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        assert!(css.contains(":focus"));
        assert!(css.contains("hyphens:none"));
    }

    #[test]
    fn test_multiple_typography_utilities() {
        // Test combining multiple typography utilities
        let mut builder = TailwindBuilder::default();
        builder.trace("line-clamp-2", false).unwrap();
        builder.trace("hyphens-auto", false).unwrap();
        builder.trace("truncate", false).unwrap();  // Use truncate instead of text-ellipsis
        let css = builder.bundle().unwrap_or_default();
        
        // Check line-clamp styles
        assert!(css.contains("-webkit-line-clamp:2"));
        
        // Check hyphens styles
        assert!(css.contains("hyphens:auto"));
        
        // Check truncate styles (text-overflow:ellipsis)
        assert!(css.contains("text-overflow:ellipsis"));
    }

    #[test]
    fn test_line_clamp_css_format() {
        let mut builder = TailwindBuilder::default();
        builder.trace("line-clamp-3", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        
        // Verify the CSS is properly formatted
        assert!(css.contains(".line-clamp-3"));
        assert!(css.contains("overflow:hidden"));
        assert!(css.contains("display:-webkit-box"));
        assert!(css.contains("-webkit-box-orient:vertical"));
        assert!(css.contains("-webkit-line-clamp:3"));
    }

    #[test]
    fn test_hyphens_css_format() {
        let mut builder = TailwindBuilder::default();
        builder.trace("hyphens-manual", false).unwrap();
        let css = builder.bundle().unwrap_or_default();
        
        // Verify the CSS is properly formatted
        assert!(css.contains(".hyphens-manual"));
        assert!(css.contains("hyphens:manual"));
    }
}