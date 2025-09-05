use tailwind_css::TailwindBuilder;

#[test]
fn test_scroll_margin_logical_properties() {
    let mut builder = TailwindBuilder::default();
    
    // Test basic values
    let classes = [
        ("scroll-ms-0", "scroll-margin-inline-start: 0rem"),
        ("scroll-ms-1", "scroll-margin-inline-start: 0.25rem"),
        ("scroll-ms-2", "scroll-margin-inline-start: 0.5rem"),
        ("scroll-ms-4", "scroll-margin-inline-start: 1rem"),
        ("scroll-ms-8", "scroll-margin-inline-start: 2rem"),
        ("scroll-ms-16", "scroll-margin-inline-start: 4rem"),
        ("scroll-ms-32", "scroll-margin-inline-start: 8rem"),
        ("scroll-ms-64", "scroll-margin-inline-start: 16rem"),
        ("scroll-ms-96", "scroll-margin-inline-start: 24rem"),
        ("scroll-ms-px", "scroll-margin-inline-start: 1px"),
        
        ("scroll-me-0", "scroll-margin-inline-end: 0rem"),
        ("scroll-me-1", "scroll-margin-inline-end: 0.25rem"),
        ("scroll-me-2", "scroll-margin-inline-end: 0.5rem"),
        ("scroll-me-4", "scroll-margin-inline-end: 1rem"),
        ("scroll-me-8", "scroll-margin-inline-end: 2rem"),
        ("scroll-me-16", "scroll-margin-inline-end: 4rem"),
        ("scroll-me-32", "scroll-margin-inline-end: 8rem"),
        ("scroll-me-64", "scroll-margin-inline-end: 16rem"),
        ("scroll-me-96", "scroll-margin-inline-end: 24rem"),
        ("scroll-me-px", "scroll-margin-inline-end: 1px"),
    ];

    for (class, expected_css) in classes {
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        // Remove whitespace for comparison
        let normalized_css = css.replace('\n', "").replace(" ", "").replace(";}", "}");
        let normalized_expected = format!(".{}{{{}}}", 
            class, 
            expected_css.replace(" ", "")
        );
        
        assert!(
            normalized_css.contains(&normalized_expected),
            "Class '{}' should generate CSS containing '{}', but got '{}'",
            class,
            normalized_expected,
            normalized_css
        );
        
        builder.clear();
    }
}

#[test]
fn test_scroll_padding_logical_properties() {
    let mut builder = TailwindBuilder::default();
    
    // Test basic values
    let classes = [
        ("scroll-ps-0", "scroll-padding-inline-start: 0rem"),
        ("scroll-ps-1", "scroll-padding-inline-start: 0.25rem"),
        ("scroll-ps-2", "scroll-padding-inline-start: 0.5rem"),
        ("scroll-ps-4", "scroll-padding-inline-start: 1rem"),
        ("scroll-ps-8", "scroll-padding-inline-start: 2rem"),
        ("scroll-ps-16", "scroll-padding-inline-start: 4rem"),
        ("scroll-ps-32", "scroll-padding-inline-start: 8rem"),
        ("scroll-ps-64", "scroll-padding-inline-start: 16rem"),
        ("scroll-ps-96", "scroll-padding-inline-start: 24rem"),
        ("scroll-ps-px", "scroll-padding-inline-start: 1px"),
        
        ("scroll-pe-0", "scroll-padding-inline-end: 0rem"),
        ("scroll-pe-1", "scroll-padding-inline-end: 0.25rem"),
        ("scroll-pe-2", "scroll-padding-inline-end: 0.5rem"),
        ("scroll-pe-4", "scroll-padding-inline-end: 1rem"),
        ("scroll-pe-8", "scroll-padding-inline-end: 2rem"),
        ("scroll-pe-16", "scroll-padding-inline-end: 4rem"),
        ("scroll-pe-32", "scroll-padding-inline-end: 8rem"),
        ("scroll-pe-64", "scroll-padding-inline-end: 16rem"),
        ("scroll-pe-96", "scroll-padding-inline-end: 24rem"),
        ("scroll-pe-px", "scroll-padding-inline-end: 1px"),
    ];

    for (class, expected_css) in classes {
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        // Remove whitespace for comparison
        let normalized_css = css.replace('\n', "").replace(" ", "").replace(";}", "}");
        let normalized_expected = format!(".{}{{{}}}", 
            class, 
            expected_css.replace(" ", "")
        );
        
        assert!(
            normalized_css.contains(&normalized_expected),
            "Class '{}' should generate CSS containing '{}', but got '{}'",
            class,
            normalized_expected,
            normalized_css
        );
        
        builder.clear();
    }
}

#[test]
fn test_scroll_margin_logical_arbitrary() {
    let mut builder = TailwindBuilder::default();
    
    // Test arbitrary values
    let classes = [
        ("scroll-ms-[10px]", "scroll-margin-inline-start: 10px"),
        ("scroll-ms-[2.5rem]", "scroll-margin-inline-start: 2.5rem"),
        ("scroll-ms-[50%]", "scroll-margin-inline-start: 50%"),
        ("scroll-ms-[1em]", "scroll-margin-inline-start: 1em"),
        ("scroll-ms-[100vh]", "scroll-margin-inline-start: 100vh"),
        
        ("scroll-me-[10px]", "scroll-margin-inline-end: 10px"),
        ("scroll-me-[2.5rem]", "scroll-margin-inline-end: 2.5rem"),
        ("scroll-me-[50%]", "scroll-margin-inline-end: 50%"),
        ("scroll-me-[1em]", "scroll-margin-inline-end: 1em"),
        ("scroll-me-[100vh]", "scroll-margin-inline-end: 100vh"),
    ];

    for (class, expected_css) in classes {
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        // Remove whitespace for comparison
        let normalized_css = css.replace('\n', "").replace(" ", "").replace(";}", "}");
        // For arbitrary values, the class name in CSS will be escaped
        let escaped_class = class.replace("[", "\\[").replace("]", "\\]").replace("%", "\\%").replace(".", "\\.");
        let normalized_expected = format!(".{}{{{}}}", 
            escaped_class, 
            expected_css.replace(" ", "")
        );
        
        assert!(
            normalized_css.contains(&normalized_expected),
            "Class '{}' should generate CSS containing '{}', but got '{}'",
            class,
            normalized_expected,
            normalized_css
        );
        
        builder.clear();
    }
}

#[test]
fn test_scroll_padding_logical_arbitrary() {
    let mut builder = TailwindBuilder::default();
    
    // Test arbitrary values
    let classes = [
        ("scroll-ps-[10px]", "scroll-padding-inline-start: 10px"),
        ("scroll-ps-[2.5rem]", "scroll-padding-inline-start: 2.5rem"),
        ("scroll-ps-[50%]", "scroll-padding-inline-start: 50%"),
        ("scroll-ps-[1em]", "scroll-padding-inline-start: 1em"),
        ("scroll-ps-[100vh]", "scroll-padding-inline-start: 100vh"),
        
        ("scroll-pe-[10px]", "scroll-padding-inline-end: 10px"),
        ("scroll-pe-[2.5rem]", "scroll-padding-inline-end: 2.5rem"),
        ("scroll-pe-[50%]", "scroll-padding-inline-end: 50%"),
        ("scroll-pe-[1em]", "scroll-padding-inline-end: 1em"),
        ("scroll-pe-[100vh]", "scroll-padding-inline-end: 100vh"),
    ];

    for (class, expected_css) in classes {
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        // Remove whitespace for comparison
        let normalized_css = css.replace('\n', "").replace(" ", "").replace(";}", "}");
        // For arbitrary values, the class name in CSS will be escaped
        let escaped_class = class.replace("[", "\\[").replace("]", "\\]").replace("%", "\\%").replace(".", "\\.");
        let normalized_expected = format!(".{}{{{}}}", 
            escaped_class, 
            expected_css.replace(" ", "")
        );
        
        assert!(
            normalized_css.contains(&normalized_expected),
            "Class '{}' should generate CSS containing '{}', but got '{}'",
            class,
            normalized_expected,
            normalized_css
        );
        
        builder.clear();
    }
}

#[test]
fn test_scroll_logical_with_modifiers() {
    let mut builder = TailwindBuilder::default();
    
    // Test with hover modifier
    let class = "hover:scroll-ms-4";
    builder.trace(class, false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(
        css.contains("hover:") && css.contains("scroll-margin-inline-start"),
        "Hover modifier should work with logical scroll properties"
    );
    
    builder.clear();
    
    // Test with responsive modifier
    let class = "md:scroll-ps-8";
    builder.trace(class, false).unwrap();
    let css = builder.bundle().unwrap();
    
    assert!(
        css.contains("@media") && css.contains("scroll-padding-inline-start"),
        "Responsive modifier should work with logical scroll properties"
    );
}

#[test]
fn test_scroll_logical_negative_values() {
    let mut builder = TailwindBuilder::default();
    builder.preflight.disable = true; // Disable reset styles for test
    
    // Test negative values for scroll margins
    let classes = [
        ("-scroll-ms-4", "scroll-margin-inline-start: -1rem"),
        ("-scroll-ms-8", "scroll-margin-inline-start: -2rem"),
        ("-scroll-me-4", "scroll-margin-inline-end: -1rem"),
        ("-scroll-me-8", "scroll-margin-inline-end: -2rem"),
    ];

    for (class, expected_css) in classes {
        builder.trace(class, false).unwrap();
        let css = builder.bundle().unwrap();
        
        // Remove whitespace for comparison
        let normalized_css = css.replace('\n', "").replace(" ", "").replace(";}", "}");
        // Use the class name as-is, no escaping needed
        let normalized_expected = format!(".{}{{{}}}", 
            class, 
            expected_css.replace(" ", "")
        );
        
        assert!(
            normalized_css.contains(&normalized_expected),
            "Class '{}' should generate CSS containing '{}', but got '{}'",
            class,
            normalized_expected,
            normalized_css
        );
        
        builder.clear();
    }
}