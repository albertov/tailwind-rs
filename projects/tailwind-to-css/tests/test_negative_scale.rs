use tailwind_css::TailwindBuilder;

#[test]
fn test_negative_scale_transforms() {
    let mut builder = TailwindBuilder::default();

    // Test negative scale-50
    let result = builder.trace("-scale-50", false);
    println!("Trace result for -scale-50: {:?}", result);
    let css = builder.bundle().unwrap_or_default();
    println!("CSS output for -scale-50:\n{}", css);
    
    if css.is_empty() {
        println!("WARNING: CSS is empty, class might not be recognized");
    }
    
    // Check if the class is being parsed at all
    assert!(!css.is_empty(), "CSS should not be empty for -scale-50");
    assert!(css.contains("transform"), "CSS should contain transform property, got: {}", css);
    assert!(css.contains("-0.5"), "CSS should contain -0.5 value, got: {}", css);
}