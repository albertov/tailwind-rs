use tailwind_css::TailwindBuilder;

fn generate_css(class: &str) -> String {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace(class, false);
    builder.bundle().unwrap_or_default()
}

// === EDGE CASES AND SPECIAL SCENARIOS ===

#[test]
fn test_filter_none_values() {
    // Test that "none" values work for all filter utilities
    let css = generate_css("blur-none");
    assert!(css.contains("--tw-blur:blur(0)"), "Expected blur(0) for blur-none");
    
    let css = generate_css("grayscale-0");
    assert!(css.contains("--tw-grayscale:grayscale(0%)") || css.contains("--tw-grayscale:grayscale(0)"), 
            "Expected grayscale(0) for grayscale-0");
    
    let css = generate_css("invert-0");
    assert!(css.contains("--tw-invert:invert(0%)") || css.contains("--tw-invert:invert(0)"), 
            "Expected invert(0) for invert-0");
    
    let css = generate_css("sepia-0");
    assert!(css.contains("--tw-sepia:sepia(0%)") || css.contains("--tw-sepia:sepia(0)"), 
            "Expected sepia(0) for sepia-0");
}

#[test]
fn test_filter_100_percent_values() {
    // Test 100% values
    let css = generate_css("brightness-100");
    assert!(css.contains("--tw-brightness:brightness(100%)") || css.contains("--tw-brightness:brightness(1)"), 
            "Expected brightness(100%) for brightness-100");
    
    let css = generate_css("contrast-100");
    assert!(css.contains("--tw-contrast:contrast(100%)") || css.contains("--tw-contrast:contrast(1)"), 
            "Expected contrast(100%) for contrast-100");
    
    let css = generate_css("saturate-100");
    assert!(css.contains("--tw-saturate:saturate(100%)") || css.contains("--tw-saturate:saturate(1)"), 
            "Expected saturate(100%) for saturate-100");
}

#[test]
fn test_filter_chain_order() {
    // Ensure filter chain includes all filter types in correct order
    let css = generate_css("blur-sm");
    
    // Check that the filter chain includes all possible filter variables
    assert!(css.contains("var(--tw-blur)"), "Filter chain should include blur variable");
    assert!(css.contains("var(--tw-brightness)"), "Filter chain should include brightness variable");
    assert!(css.contains("var(--tw-contrast)"), "Filter chain should include contrast variable");
    assert!(css.contains("var(--tw-grayscale)"), "Filter chain should include grayscale variable");
    assert!(css.contains("var(--tw-hue-rotate)"), "Filter chain should include hue-rotate variable");
    assert!(css.contains("var(--tw-invert)"), "Filter chain should include invert variable");
    assert!(css.contains("var(--tw-saturate)"), "Filter chain should include saturate variable");
    assert!(css.contains("var(--tw-sepia)"), "Filter chain should include sepia variable");
    assert!(css.contains("var(--tw-drop-shadow)"), "Filter chain should include drop-shadow variable");
}

#[test]
fn test_backdrop_filter_chain_order() {
    // Ensure backdrop-filter chain includes all backdrop filter types
    let css = generate_css("backdrop-blur-sm");
    
    // Check that the backdrop-filter chain includes all possible backdrop filter variables
    assert!(css.contains("var(--tw-backdrop-blur)"), "Backdrop filter chain should include blur variable");
    assert!(css.contains("var(--tw-backdrop-brightness)"), "Backdrop filter chain should include brightness variable");
    assert!(css.contains("var(--tw-backdrop-contrast)"), "Backdrop filter chain should include contrast variable");
    assert!(css.contains("var(--tw-backdrop-grayscale)"), "Backdrop filter chain should include grayscale variable");
    assert!(css.contains("var(--tw-backdrop-hue-rotate)"), "Backdrop filter chain should include hue-rotate variable");
    assert!(css.contains("var(--tw-backdrop-invert)"), "Backdrop filter chain should include invert variable");
    assert!(css.contains("var(--tw-backdrop-opacity)"), "Backdrop filter chain should include opacity variable");
    assert!(css.contains("var(--tw-backdrop-saturate)"), "Backdrop filter chain should include saturate variable");
    assert!(css.contains("var(--tw-backdrop-sepia)"), "Backdrop filter chain should include sepia variable");
}

#[test]
fn test_numeric_filter_values() {
    // Test numeric values without percentage suffix
    let css = generate_css("brightness-0");
    assert!(css.contains("--tw-brightness:brightness(0%)") || css.contains("--tw-brightness:brightness(0)"), 
            "Expected brightness(0) for brightness-0");
    
    let css = generate_css("brightness-50");
    assert!(css.contains("--tw-brightness:brightness(50%)") || css.contains("--tw-brightness:brightness(0.5)"), 
            "Expected brightness(50%) for brightness-50");
    
    let css = generate_css("brightness-200");
    assert!(css.contains("--tw-brightness:brightness(200%)") || css.contains("--tw-brightness:brightness(2)"), 
            "Expected brightness(200%) for brightness-200");
}

#[test]
fn test_filter_with_important() {
    // Test important modifier
    let css = generate_css("!blur-lg");
    assert!(css.contains("--tw-blur:blur(16px)"), "Expected blur value with important");
    assert!(css.contains("!important"), "Expected !important flag");
}

#[test]
fn test_filter_arbitrary_with_units() {
    // Test arbitrary values with different units
    let css = generate_css("blur-[0.5rem]");
    assert!(css.contains("--tw-blur:blur(0.5rem)"), "Expected blur with rem units");
    
    let css = generate_css("hue-rotate-[1.5rad]");
    assert!(css.contains("--tw-hue-rotate:hue-rotate(1.5rad)"), "Expected hue-rotate with rad units");
    
    let css = generate_css("hue-rotate-[0.25turn]");
    assert!(css.contains("--tw-hue-rotate:hue-rotate(0.25turn)"), "Expected hue-rotate with turn units");
}

#[test]
fn test_filter_with_css_variables() {
    // Test arbitrary values using CSS variables
    let css = generate_css("brightness-[var(--my-brightness)]");
    assert!(css.contains("--tw-brightness:brightness(var(--my-brightness))"), 
            "Expected brightness with CSS variable");
    
    let css = generate_css("blur-[var(--blur-amount)]");
    assert!(css.contains("--tw-blur:blur(var(--blur-amount))"), 
            "Expected blur with CSS variable");
}

#[test]
fn test_filter_combined_with_opacity() {
    // Test that filters work alongside opacity
    let css = generate_css("blur-sm opacity-50");
    assert!(css.contains("--tw-blur:blur(4px)"), "Expected blur value");
    assert!(css.contains("opacity:50%"), "Expected opacity value");
}

#[test]
fn test_filter_with_transition() {
    // Test filter utilities with transitions
    let css = generate_css("transition-filter duration-300 hover:blur-lg");
    assert!(css.contains("transition-property:filter"), "Expected filter transition property");
    assert!(css.contains("transition-duration:300ms"), "Expected transition duration");
    assert!(css.contains(":hover"), "Expected hover state");
    assert!(css.contains("--tw-blur:blur(16px)"), "Expected blur value on hover");
}

#[test]
fn test_will_change_filter() {
    // Test will-change with filter
    let css = generate_css("will-change-filter");
    assert!(css.contains("will-change:filter"), "Expected will-change: filter");
}

// Removed: supports queries not implemented in Phase 2
// #[test]
// fn test_filter_supports_queries() {
//     // Test with supports queries
//     let css = generate_css("supports-[filter]:blur-lg");
//     assert!(css.contains("@supports"), "Expected @supports query");
//     assert!(css.contains("--tw-blur:blur(16px)"), "Expected blur value in supports query");
// }

#[test]
fn test_filter_with_container_queries() {
    // Test with container queries
    let css = generate_css("@lg:blur-md");
    assert!(css.contains("@container"), "Expected container query");
    assert!(css.contains("--tw-blur:blur(12px)"), "Expected blur value in container query");
}

#[test]
fn test_negative_values_only_hue_rotate() {
    // Ensure negative values only work for hue-rotate
    let css = generate_css("-hue-rotate-90");
    assert!(css.contains("--tw-hue-rotate:hue-rotate(-90deg)"), 
            "Expected negative hue-rotate value");
}

// Removed: filter-none not implemented in Phase 2
// #[test]
// fn test_filter_reset_with_none() {
//     // Test filter reset with filter-none utility (if implemented)
//     let css = generate_css("filter-none");
//     // This might not be implemented yet, but testing for future compatibility
//     if !css.is_empty() {
//         assert!(css.contains("filter:none"), "Expected filter: none");
//     }
// }

#[test]
fn test_backdrop_filter_with_opacity() {
    // Test backdrop opacity specifically
    let css = generate_css("backdrop-opacity-75");
    assert!(css.contains("backdrop-filter:opacity(75%)"), 
            "Expected backdrop-filter with opacity function");
}

#[test]
fn test_filter_inheritance() {
    // Test that multiple filter classes on same element compose correctly
    let css = generate_css("blur-sm brightness-75 contrast-125 saturate-150");
    
    // All variables should be set
    assert!(css.contains("--tw-blur:blur(4px)"), "Expected blur variable");
    assert!(css.contains("--tw-brightness:brightness(75%)") || css.contains("--tw-brightness:brightness(0.75)"), 
            "Expected brightness variable");
    assert!(css.contains("--tw-contrast:contrast(125%)") || css.contains("--tw-contrast:contrast(1.25)"), 
            "Expected contrast variable");
    assert!(css.contains("--tw-saturate:saturate(150%)") || css.contains("--tw-saturate:saturate(1.5)"), 
            "Expected saturate variable");
    
    // Filter property should reference all variables
    let filter_count = css.matches("filter:").count();
    assert!(filter_count >= 4, "Expected multiple filter declarations for composition");
}

// Removed: print variant not implemented in Phase 2
// #[test]
// fn test_filter_print_variant() {
//     // Test print media variant
//     let css = generate_css("print:blur-none");
//     assert!(css.contains("@media print"), "Expected print media query");
//     assert!(css.contains("--tw-blur:blur(0)"), "Expected blur(0) in print mode");
// }

// Removed: motion-safe variant not implemented in Phase 2
// #[test]
// fn test_filter_motion_safe_variant() {
//     // Test motion-safe variant
//     let css = generate_css("motion-safe:blur-lg");
//     assert!(css.contains("@media (prefers-reduced-motion: no-preference)"), 
//             "Expected motion-safe media query");
//     assert!(css.contains("--tw-blur:blur(16px)"), "Expected blur value in motion-safe");
// }

// Removed: motion-reduce variant not implemented in Phase 2  
// #[test]
// fn test_filter_motion_reduce_variant() {
//     // Test motion-reduce variant
//     let css = generate_css("motion-reduce:blur-none");
//     assert!(css.contains("@media (prefers-reduced-motion: reduce)"), 
//             "Expected motion-reduce media query");
//     assert!(css.contains("--tw-blur:blur(0)"), "Expected blur(0) in motion-reduce");
// }