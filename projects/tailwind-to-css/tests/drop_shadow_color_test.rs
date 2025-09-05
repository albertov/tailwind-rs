use tailwind_css::TailwindBuilder;

fn generate_css(class: &str) -> String {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace(class, false);
    builder.bundle().unwrap_or_default()
}

// === DROP-SHADOW COLOR TESTS ===
// Testing if drop-shadow supports color modifiers like Tailwind CSS v3/v4

#[test]
fn test_drop_shadow_with_color() {
    // In Tailwind CSS, drop-shadow doesn't directly support color modifiers
    // Colors are part of the drop-shadow value itself
    let css = generate_css("drop-shadow-lg");
    
    // Check that the drop-shadow includes the color in its value
    assert!(css.contains("rgb(0 0 0") || css.contains("rgba(0"), 
            "Drop-shadow should include color in its value, got: {}", css);
}

#[test]
fn test_drop_shadow_custom_with_color() {
    // Test arbitrary drop-shadow with custom color
    let css = generate_css("drop-shadow-[0_4px_3px_rgb(255_0_0_/_0.5)]");
    
    // The arbitrary value should be passed through
    assert!(css.contains("0 4px 3px rgb(255 0 0 / 0.5)") || 
            css.contains("0_4px_3px_rgb(255_0_0_/_0.5)"),
            "Expected custom drop-shadow with color, got: {}", css);
}

#[test]
fn test_drop_shadow_values_include_colors() {
    // Verify that predefined drop-shadow values include colors
    
    let css = generate_css("drop-shadow-sm");
    assert!(css.contains("rgb(0 0 0") || css.contains("rgba(0"), 
            "drop-shadow-sm should include black color, got: {}", css);
    
    let css = generate_css("drop-shadow-md");
    assert!(css.contains("rgb(0 0 0") || css.contains("rgba(0"), 
            "drop-shadow-md should include black color, got: {}", css);
    
    let css = generate_css("drop-shadow-xl");
    assert!(css.contains("rgb(0 0 0") || css.contains("rgba(0"), 
            "drop-shadow-xl should include black color, got: {}", css);
}

#[test]
fn test_drop_shadow_none_transparent() {
    // drop-shadow-none should use transparent color
    let css = generate_css("drop-shadow-none");
    assert!(css.contains("#0000") || css.contains("transparent") || css.contains("rgba(0, 0, 0, 0)"), 
            "drop-shadow-none should use transparent color, got: {}", css);
}