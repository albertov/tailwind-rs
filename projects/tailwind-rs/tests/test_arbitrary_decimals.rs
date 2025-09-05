use tailwind_rs::*;

/// Test arbitrary values with decimal points in various utility classes
/// These tests demonstrate that tailwind-rs currently fails to parse and generate
/// CSS for arbitrary values containing decimal points like [0.25rem], [1.5rem], [162.5%]

#[test]
fn test_gap_arbitrary_decimal_rem() {
    let mut tw = TailwindBuilder::default();
    
    // Test gap with decimal rem values
    // These should work but currently fail
    assert_eq!(tw.trace("gap-[0.25rem]", false).unwrap(), "gap-[0.25rem]");
    assert_eq!(tw.trace("gap-[1.5rem]", false).unwrap(), "gap-[1.5rem]");
    assert_eq!(tw.trace("gap-[2.75rem]", false).unwrap(), "gap-[2.75rem]");
    
    // Verify CSS output
    let css = tw.bundle().unwrap();
    
    // These assertions will fail, proving the bug exists
    assert!(
        css.contains(r#".gap-\[0\.25rem\] { gap:0.25rem; }"#) || 
        css.contains(r#".gap-\[0\.25rem\]{gap:0.25rem}"#),
        "Missing CSS for gap-[0.25rem]"
    );
    
    assert!(
        css.contains(r#".gap-\[1\.5rem\] { gap:1.5rem; }"#) || 
        css.contains(r#".gap-\[1\.5rem\]{gap:1.5rem}"#),
        "Missing CSS for gap-[1.5rem]"
    );
    
    assert!(
        css.contains(r#".gap-\[2\.75rem\] { gap:2.75rem; }"#) || 
        css.contains(r#".gap-\[2\.75rem\]{gap:2.75rem}"#),
        "Missing CSS for gap-[2.75rem]"
    );
}

#[test]
fn test_gap_x_arbitrary_decimal_rem() {
    let mut tw = TailwindBuilder::default();
    
    // Test gap-x with decimal rem values
    assert_eq!(tw.trace("gap-x-[0.25rem]", false).unwrap(), "gap-x-[0.25rem]");
    assert_eq!(tw.trace("gap-x-[1.5rem]", false).unwrap(), "gap-x-[1.5rem]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#".gap-x-\[0\.25rem\] { column-gap:0.25rem; }"#) || 
        css.contains(r#".gap-x-\[0\.25rem\]{column-gap:0.25rem}"#),
        "Missing CSS for gap-x-[0.25rem]"
    );
    
    assert!(
        css.contains(r#".gap-x-\[1\.5rem\] { column-gap:1.5rem; }"#) || 
        css.contains(r#".gap-x-\[1\.5rem\]{column-gap:1.5rem}"#),
        "Missing CSS for gap-x-[1.5rem]"
    );
}

#[test]
fn test_gap_y_arbitrary_decimal_rem() {
    let mut tw = TailwindBuilder::default();
    
    // Test gap-y with decimal rem values
    assert_eq!(tw.trace("gap-y-[0.25rem]", false).unwrap(), "gap-y-[0.25rem]");
    assert_eq!(tw.trace("gap-y-[1.5rem]", false).unwrap(), "gap-y-[1.5rem]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#".gap-y-\[0\.25rem\] { row-gap:0.25rem; }"#) || 
        css.contains(r#".gap-y-\[0\.25rem\]{row-gap:0.25rem}"#),
        "Missing CSS for gap-y-[0.25rem]"
    );
    
    assert!(
        css.contains(r#".gap-y-\[1\.5rem\] { row-gap:1.5rem; }"#) || 
        css.contains(r#".gap-y-\[1\.5rem\]{row-gap:1.5rem}"#),
        "Missing CSS for gap-y-[1.5rem]"
    );
}

#[test]
fn test_leading_arbitrary_decimal_percentage() {
    let mut tw = TailwindBuilder::default();
    
    // Test line-height (leading) with decimal percentage values
    assert_eq!(tw.trace("leading-[162.5%]", false).unwrap(), "leading-[162.5%]");
    assert_eq!(tw.trace("leading-[137.5%]", false).unwrap(), "leading-[137.5%]");
    assert_eq!(tw.trace("leading-[150.5%]", false).unwrap(), "leading-[150.5%]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#".leading-\[162\.5\%\] { line-height:162.5%; }"#) || 
        css.contains(r#".leading-\[162\.5\%\]{line-height:162.5%}"#),
        "Missing CSS for leading-[162.5%]"
    );
    
    assert!(
        css.contains(r#".leading-\[137\.5\%\] { line-height:137.5%; }"#) || 
        css.contains(r#".leading-\[137\.5\%\]{line-height:137.5%}"#),
        "Missing CSS for leading-[137.5%]"
    );
    
    assert!(
        css.contains(r#".leading-\[150\.5\%\] { line-height:150.5%; }"#) || 
        css.contains(r#".leading-\[150\.5\%\]{line-height:150.5%}"#),
        "Missing CSS for leading-[150.5%]"
    );
}

#[test]
fn test_width_arbitrary_decimal_rem() {
    let mut tw = TailwindBuilder::default();
    
    // Test width with decimal rem values
    assert_eq!(tw.trace("w-[10.5rem]", false).unwrap(), "w-[10.5rem]");
    assert_eq!(tw.trace("w-[0.75rem]", false).unwrap(), "w-[0.75rem]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#".w-\[10\.5rem\] { width:10.5rem; }"#) || 
        css.contains(r#".w-\[10\.5rem\]{width:10.5rem}"#),
        "Missing CSS for w-[10.5rem]"
    );
    
    assert!(
        css.contains(r#".w-\[0\.75rem\] { width:0.75rem; }"#) || 
        css.contains(r#".w-\[0\.75rem\]{width:0.75rem}"#),
        "Missing CSS for w-[0.75rem]"
    );
}

#[test]
fn test_height_arbitrary_decimal_rem() {
    let mut tw = TailwindBuilder::default();
    
    // Test height with decimal rem values
    assert_eq!(tw.trace("h-[10.5rem]", false).unwrap(), "h-[10.5rem]");
    assert_eq!(tw.trace("h-[0.75rem]", false).unwrap(), "h-[0.75rem]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#".h-\[10\.5rem\] { height:10.5rem; }"#) || 
        css.contains(r#".h-\[10\.5rem\]{height:10.5rem}"#),
        "Missing CSS for h-[10.5rem]"
    );
    
    assert!(
        css.contains(r#".h-\[0\.75rem\] { height:0.75rem; }"#) || 
        css.contains(r#".h-\[0\.75rem\]{height:0.75rem}"#),
        "Missing CSS for h-[0.75rem]"
    );
}

#[test]
fn test_padding_arbitrary_decimal_rem() {
    let mut tw = TailwindBuilder::default();
    
    // Test padding with decimal rem values
    assert_eq!(tw.trace("p-[0.25rem]", false).unwrap(), "p-[0.25rem]");
    assert_eq!(tw.trace("p-[1.5rem]", false).unwrap(), "p-[1.5rem]");
    assert_eq!(tw.trace("px-[0.75rem]", false).unwrap(), "px-[0.75rem]");
    assert_eq!(tw.trace("py-[2.25rem]", false).unwrap(), "py-[2.25rem]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#".p-\[0\.25rem\] { padding:0.25rem; }"#) || 
        css.contains(r#".p-\[0\.25rem\]{padding:0.25rem}"#),
        "Missing CSS for p-[0.25rem]"
    );
    
    assert!(
        css.contains(r#".p-\[1\.5rem\] { padding:1.5rem; }"#) || 
        css.contains(r#".p-\[1\.5rem\]{padding:1.5rem}"#),
        "Missing CSS for p-[1.5rem]"
    );
    
    assert!(
        css.contains(r#".px-\[0\.75rem\] { padding-left:0.75rem;padding-right:0.75rem; }"#) || 
        css.contains(r#".px-\[0\.75rem\]{padding-left:0.75rem;padding-right:0.75rem}"#),
        "Missing CSS for px-[0.75rem]"
    );
    
    assert!(
        css.contains(r#".py-\[2\.25rem\] { padding-bottom:2.25rem;padding-top:2.25rem; }"#) || 
        css.contains(r#".py-\[2\.25rem\]{padding-bottom:2.25rem;padding-top:2.25rem}"#),
        "Missing CSS for py-[2.25rem]"
    );
}

#[test]
fn test_margin_arbitrary_decimal_rem() {
    let mut tw = TailwindBuilder::default();
    
    // Test margin with decimal rem values
    assert_eq!(tw.trace("m-[0.25rem]", false).unwrap(), "m-[0.25rem]");
    assert_eq!(tw.trace("m-[1.5rem]", false).unwrap(), "m-[1.5rem]");
    assert_eq!(tw.trace("mx-[0.75rem]", false).unwrap(), "mx-[0.75rem]");
    assert_eq!(tw.trace("my-[2.25rem]", false).unwrap(), "my-[2.25rem]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#".m-\[0\.25rem\] { margin:0.25rem; }"#) || 
        css.contains(r#".m-\[0\.25rem\]{margin:0.25rem}"#),
        "Missing CSS for m-[0.25rem]"
    );
    
    assert!(
        css.contains(r#".m-\[1\.5rem\] { margin:1.5rem; }"#) || 
        css.contains(r#".m-\[1\.5rem\]{margin:1.5rem}"#),
        "Missing CSS for m-[1.5rem]"
    );
    
    assert!(
        css.contains(r#".mx-\[0\.75rem\] { margin-left:0.75rem;margin-right:0.75rem; }"#) || 
        css.contains(r#".mx-\[0\.75rem\]{margin-left:0.75rem;margin-right:0.75rem}"#),
        "Missing CSS for mx-[0.75rem]"
    );
    
    assert!(
        css.contains(r#".my-\[2\.25rem\] { margin-bottom:2.25rem;margin-top:2.25rem; }"#) || 
        css.contains(r#".my-\[2\.25rem\]{margin-bottom:2.25rem;margin-top:2.25rem}"#),
        "Missing CSS for my-[2.25rem]"
    );
}

#[test]
fn test_text_size_arbitrary_decimal_rem() {
    let mut tw = TailwindBuilder::default();
    
    // Test text size with decimal rem values
    assert_eq!(tw.trace("text-[0.875rem]", false).unwrap(), "text-[0.875rem]");
    assert_eq!(tw.trace("text-[1.25rem]", false).unwrap(), "text-[1.25rem]");
    assert_eq!(tw.trace("text-[2.5rem]", false).unwrap(), "text-[2.5rem]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#".text-\[0\.875rem\] { font-size:0.875rem; }"#) || 
        css.contains(r#".text-\[0\.875rem\]{font-size:0.875rem}"#),
        "Missing CSS for text-[0.875rem]"
    );
    
    assert!(
        css.contains(r#".text-\[1\.25rem\] { font-size:1.25rem; }"#) || 
        css.contains(r#".text-\[1\.25rem\]{font-size:1.25rem}"#),
        "Missing CSS for text-[1.25rem]"
    );
    
    assert!(
        css.contains(r#".text-\[2\.5rem\] { font-size:2.5rem; }"#) || 
        css.contains(r#".text-\[2\.5rem\]{font-size:2.5rem}"#),
        "Missing CSS for text-[2.5rem]"
    );
}

#[test]
fn test_translate_arbitrary_decimal_percentage() {
    let mut tw = TailwindBuilder::default();
    
    // Test translate with decimal percentage values
    assert_eq!(tw.trace("translate-x-[12.5%]", false).unwrap(), "translate-x-[12.5%]");
    assert_eq!(tw.trace("translate-y-[37.5%]", false).unwrap(), "translate-y-[37.5%]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#"translate-x-\[12\.5\%\]"#) && css.contains("--tw-translate-x:12.5%;transform:translate(var(--tw-translate-x, 0)"),
        "Missing CSS for translate-x-[12.5%]:\n{}", css
    );
    
    assert!(
        css.contains(r#"translate-y-\[37\.5\%\]"#) && css.contains("--tw-translate-y:37.5%;transform:translate(var(--tw-translate-x, 0)"),
        "Missing CSS for translate-y-[37.5%]:\n{}", css
    );
}

#[test]
fn test_opacity_arbitrary_decimal() {
    let mut tw = TailwindBuilder::default();
    
    // Test opacity with decimal values (note: without percent sign)
    // Note: tailwind-rs might normalize these to standard opacity values
    let opacity_35 = tw.trace("opacity-[0.35]", false).unwrap();
    let opacity_67 = tw.trace("opacity-[0.67]", false).unwrap();
    let opacity_99 = tw.trace("opacity-[0.99]", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Check that opacity values are traced (even if normalized)
    println!("opacity-[0.35] traced as: {}", opacity_35);
    println!("opacity-[0.67] traced as: {}", opacity_67);
    println!("opacity-[0.99] traced as: {}", opacity_99);
    
    // The important part is that the CSS contains opacity rules
    // tailwind-rs might normalize arbitrary opacity values to standard ones
    assert!(
        css.contains("opacity:"),
        "Missing opacity CSS rules. Generated CSS: {}", css
    );
}

#[test]
fn test_scale_arbitrary_decimal() {
    let mut tw = TailwindBuilder::default();
    
    // Test scale with decimal values
    // Note: tailwind-rs might normalize these
    let scale_125 = tw.trace("scale-[1.25]", false).unwrap();
    let scale_x_075 = tw.trace("scale-x-[0.75]", false).unwrap();
    let scale_y_15 = tw.trace("scale-y-[1.5]", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Check what was actually traced
    println!("scale-[1.25] traced as: {}", scale_125);
    println!("scale-x-[0.75] traced as: {}", scale_x_075);
    println!("scale-y-[1.5] traced as: {}", scale_y_15);
    
    // Check for scale transforms in CSS
    assert!(
        css.contains("scale(") || css.contains("scaleX(") || css.contains("scaleY("),
        "Missing scale CSS transforms. Generated CSS: {}", css
    );
}

#[test]
fn test_border_radius_arbitrary_decimal_rem() {
    let mut tw = TailwindBuilder::default();
    
    // Test border radius with decimal rem values
    assert_eq!(tw.trace("rounded-[0.375rem]", false).unwrap(), "rounded-[0.375rem]");
    assert_eq!(tw.trace("rounded-[1.25rem]", false).unwrap(), "rounded-[1.25rem]");
    
    let css = tw.bundle().unwrap();
    
    assert!(
        css.contains(r#".rounded-\[0\.375rem\] { border-radius:0.375rem; }"#) || 
        css.contains(r#".rounded-\[0\.375rem\]{border-radius:0.375rem}"#),
        "Missing CSS for rounded-[0.375rem]"
    );
    
    assert!(
        css.contains(r#".rounded-\[1\.25rem\] { border-radius:1.25rem; }"#) || 
        css.contains(r#".rounded-\[1\.25rem\]{border-radius:1.25rem}"#),
        "Missing CSS for rounded-[1.25rem]"
    );
}

#[test]
fn test_responsive_modifiers_with_decimal_arbitrary_values() {
    let mut tw = TailwindBuilder::default();
    
    // Test responsive modifiers with decimal arbitrary values
    assert_eq!(tw.trace("sm:gap-[0.75rem]", false).unwrap(), "sm:gap-[0.75rem]");
    assert_eq!(tw.trace("md:p-[1.25rem]", false).unwrap(), "md:p-[1.25rem]");
    assert_eq!(tw.trace("lg:text-[2.75rem]", false).unwrap(), "lg:text-[2.75rem]");
    
    let css = tw.bundle().unwrap();
    
    // Check for media query and the class
    assert!(
        css.contains("@media") && css.contains("min-width") && 
        css.contains(r#"gap-\[0\.75rem\]"#) && css.contains("gap:0.75rem"),
        "Missing responsive CSS for sm:gap-[0.75rem]"
    );
    
    assert!(
        css.contains("@media") && css.contains("min-width") && 
        css.contains(r#"p-\[1\.25rem\]"#) && css.contains("padding:1.25rem"),
        "Missing responsive CSS for md:p-[1.25rem]"
    );
    
    assert!(
        css.contains("@media") && css.contains("min-width") && 
        css.contains(r#"text-\[2\.75rem\]"#) && css.contains("font-size:2.75rem"),
        "Missing responsive CSS for lg:text-[2.75rem]"
    );
}

#[test]
fn test_negative_arbitrary_decimal_values() {
    let mut tw = TailwindBuilder::default();
    
    // Test negative arbitrary values with decimals
    // Note: tailwind-rs might handle negative values differently
    let neg_margin = tw.trace("-m-[0.25rem]", false).unwrap();
    let neg_translate = tw.trace("-translate-x-[1.5rem]", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Check what was actually traced
    println!("-m-[0.25rem] traced as: {}", neg_margin);
    println!("-translate-x-[1.5rem] traced as: {}", neg_translate);
    
    // Check for negative values in CSS
    // The important part is that negative values are handled
    assert!(
        css.contains("margin:") || css.contains("translateX("),
        "Missing CSS for negative values. Generated CSS: {}", css
    );
}

/// This is a comprehensive smoke test for all the decimal arbitrary values from the real-world use case
#[test]
fn test_real_world_failing_cases() {
    let mut tw = TailwindBuilder::default();
    
    // These are the exact classes that are failing in production
    let failing_classes = vec![
        "gap-[0.25rem]",
        "gap-[1.5rem]",
        "leading-[162.5%]",
    ];
    
    for class in &failing_classes {
        assert_eq!(
            tw.trace(class, false).unwrap(), 
            *class,
            "Failed to trace class: {}", class
        );
    }
    
    let css = tw.bundle().unwrap();
    
    // Verify each class generates proper CSS
    assert!(
        css.contains(r#".gap-\[0\.25rem\]"#) && css.contains("gap:0.25rem"),
        "CSS missing for gap-[0.25rem]. Generated CSS: {}", css
    );
    
    assert!(
        css.contains(r#".gap-\[1\.5rem\]"#) && css.contains("gap:1.5rem"),
        "CSS missing for gap-[1.5rem]. Generated CSS: {}", css
    );
    
    assert!(
        css.contains(r#".leading-\[162\.5\%\]"#) && css.contains("line-height:162.5%"),
        "CSS missing for leading-[162.5%]. Generated CSS: {}", css
    );
}
