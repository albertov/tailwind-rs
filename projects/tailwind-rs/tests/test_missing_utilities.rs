use tailwind_rs::TailwindBuilder;

// Test for missing utilities identified in the SSR demo
// These tests will initially fail, proving the utilities are missing

#[test]
fn test_flex_shrink_0() {
    let mut tw = TailwindBuilder::default();
    
    // Test flex-shrink-0 utility (normalized to shrink-0)
    let result = tw.trace("flex-shrink-0", false);
    assert!(result.is_ok(), "flex-shrink-0 should be recognized");
    assert_eq!(result.unwrap(), "shrink-0");  // Normalized to shrink-0
    
    let css = tw.bundle().unwrap();
    assert!(
        css.contains(".shrink-0") && css.contains("flex-shrink:0"),
        "CSS should contain shrink-0 with flex-shrink:0 property"
    );
}

#[test]
fn test_hover_bg_gray_100() {
    let mut tw = TailwindBuilder::default();
    
    // Test hover:bg-gray-100 utility
    let result = tw.trace("hover:bg-gray-100", false);
    assert!(result.is_ok(), "hover:bg-gray-100 should be recognized");
    assert_eq!(result.unwrap(), "hover:bg-gray-100");
    
    let css = tw.bundle().unwrap();
    // Debug: print CSS to see what's generated
    println!("Generated CSS for hover:bg-gray-100:\n{}", css);
    
    assert!(
        css.contains(".hover\\:bg-gray-100:hover"),
        "CSS should contain hover:bg-gray-100 with :hover pseudo-class"
    );
    assert!(
        css.contains("background-color:#f3f4f6") || css.contains("background-color: #f3f4f6") || css.contains("background-color:rgb(243 244 246)") || css.contains("background-color:rgb(243,244,246)"),
        "hover:bg-gray-100 should set background-color to gray-100 (#f3f4f6 or rgb equivalent)"
    );
}

#[test]
fn test_lg_flex_row() {
    let mut tw = TailwindBuilder::default();
    
    // Test lg:flex-row utility
    let result = tw.trace("lg:flex-row", false);
    assert!(result.is_ok(), "lg:flex-row should be recognized");
    assert_eq!(result.unwrap(), "lg:flex-row");
    
    let css = tw.bundle().unwrap();
    assert!(
        css.contains("@media (min-width: 1024px)"),
        "CSS should contain lg breakpoint media query"
    );
    assert!(
        css.contains(".lg\\:flex-row") && css.contains("flex-direction:row"),
        "CSS should contain lg:flex-row with flex-direction:row property"
    );
}

#[test]
fn test_lg_w_80() {
    let mut tw = TailwindBuilder::default();
    
    // Test lg:w-80 utility
    let result = tw.trace("lg:w-80", false);
    assert!(result.is_ok(), "lg:w-80 should be recognized");
    assert_eq!(result.unwrap(), "lg:w-80");
    
    let css = tw.bundle().unwrap();
    assert!(
        css.contains("@media (min-width: 1024px)"),
        "CSS should contain lg breakpoint media query"
    );
    assert!(
        css.contains(".lg\\:w-80") && css.contains("width:20rem"),
        "CSS should contain lg:w-80 with width:20rem property"
    );
}

#[test]
fn test_all_missing_utilities_together() {
    let mut tw = TailwindBuilder::default();
    
    // Test all utilities together as they might appear in real HTML
    tw.trace("flex-shrink-0", false).unwrap();
    tw.trace("hover:bg-gray-100", false).unwrap();
    tw.trace("lg:flex-row", false).unwrap();
    tw.trace("lg:w-80", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify all utilities are present (flex-shrink-0 is normalized to shrink-0)
    assert!(css.contains(".shrink-0"), "Missing shrink-0 (from flex-shrink-0)");
    assert!(css.contains(".hover\\:bg-gray-100:hover"), "Missing hover:bg-gray-100");
    assert!(css.contains(".lg\\:flex-row"), "Missing lg:flex-row");
    assert!(css.contains(".lg\\:w-80"), "Missing lg:w-80");
}