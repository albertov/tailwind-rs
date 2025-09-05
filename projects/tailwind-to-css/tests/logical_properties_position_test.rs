use tailwind_css::TailwindBuilder;

fn generate_css(class: &str) -> String {
    let mut builder = TailwindBuilder::default();
    
    // Process the class
    let _ = builder.trace(class, false);
    
    // Get the generated CSS
    builder.bundle().unwrap_or_default()
}

#[test]
fn test_logical_position_start_numeric() {
    // Test numeric values
    let css = generate_css("start-0");
    assert!(css.contains("inset-inline-start:0rem"), "Expected inset-inline-start:0rem, got: {}", css);
    
    let css = generate_css("start-1");
    assert!(css.contains("inset-inline-start:0.25rem"), "Expected inset-inline-start:0.25rem, got: {}", css);
    
    let css = generate_css("start-2");
    assert!(css.contains("inset-inline-start:0.5rem"), "Expected inset-inline-start:0.5rem, got: {}", css);
    
    let css = generate_css("start-4");
    assert!(css.contains("inset-inline-start:1rem"), "Expected inset-inline-start:1rem, got: {}", css);
    
    let css = generate_css("start-8");
    assert!(css.contains("inset-inline-start:2rem"), "Expected inset-inline-start:2rem, got: {}", css);
    
    let css = generate_css("start-16");
    assert!(css.contains("inset-inline-start:4rem"), "Expected inset-inline-start:4rem, got: {}", css);
    
    let css = generate_css("start-32");
    assert!(css.contains("inset-inline-start:8rem"), "Expected inset-inline-start:8rem, got: {}", css);
    
    let css = generate_css("start-64");
    assert!(css.contains("inset-inline-start:16rem"), "Expected inset-inline-start:16rem, got: {}", css);
    
    let css = generate_css("start-96");
    assert!(css.contains("inset-inline-start:24rem"), "Expected inset-inline-start:24rem, got: {}", css);
}

#[test]
fn test_logical_position_start_fractional() {
    // Test fractional values
    let css = generate_css("start-0.5");
    assert!(css.contains("inset-inline-start:0.125rem"), "Expected inset-inline-start:0.125rem, got: {}", css);
    
    let css = generate_css("start-1.5");
    assert!(css.contains("inset-inline-start:0.375rem"), "Expected inset-inline-start:0.375rem, got: {}", css);
    
    let css = generate_css("start-2.5");
    assert!(css.contains("inset-inline-start:0.625rem"), "Expected inset-inline-start:0.625rem, got: {}", css);
}

#[test]
fn test_logical_position_start_percentage() {
    // Test percentage values
    let css = generate_css("start-1/2");
    assert!(css.contains("inset-inline-start:50%"), "Expected inset-inline-start:50%, got: {}", css);
    
    let css = generate_css("start-1/3");
    assert!(css.contains("inset-inline-start:33.333333%"), "Expected inset-inline-start:33.333333%, got: {}", css);
    
    let css = generate_css("start-2/3");
    assert!(css.contains("inset-inline-start:66.666667%"), "Expected inset-inline-start:66.666667%, got: {}", css);
    
    let css = generate_css("start-1/4");
    assert!(css.contains("inset-inline-start:25%"), "Expected inset-inline-start:25%, got: {}", css);
    
    let css = generate_css("start-2/4");
    assert!(css.contains("inset-inline-start:50%"), "Expected inset-inline-start:50%, got: {}", css);
    
    let css = generate_css("start-3/4");
    assert!(css.contains("inset-inline-start:75%"), "Expected inset-inline-start:75%, got: {}", css);
}

#[test]
fn test_logical_position_start_special() {
    // Test special values
    let css = generate_css("start-px");
    assert!(css.contains("inset-inline-start:1px"), "Expected inset-inline-start:1px, got: {}", css);
    
    let css = generate_css("start-full");
    assert!(css.contains("inset-inline-start:100%"), "Expected inset-inline-start:100%, got: {}", css);
    
    let css = generate_css("start-auto");
    assert!(css.contains("inset-inline-start:auto"), "Expected inset-inline-start:auto, got: {}", css);
}

#[test]
fn test_logical_position_start_negative() {
    // Test negative values
    let css = generate_css("-start-1");
    assert!(css.contains("inset-inline-start:-0.25rem"), "Expected inset-inline-start:-0.25rem, got: {}", css);
    
    let css = generate_css("-start-2");
    assert!(css.contains("inset-inline-start:-0.5rem"), "Expected inset-inline-start:-0.5rem, got: {}", css);
    
    let css = generate_css("-start-4");
    assert!(css.contains("inset-inline-start:-1rem"), "Expected inset-inline-start:-1rem, got: {}", css);
    
    let css = generate_css("-start-px");
    assert!(css.contains("inset-inline-start:-1px"), "Expected inset-inline-start:-1px, got: {}", css);
    
    let css = generate_css("-start-full");
    assert!(css.contains("inset-inline-start:-100%"), "Expected inset-inline-start:-100%, got: {}", css);
}

#[test]
fn test_logical_position_start_arbitrary() {
    // Test arbitrary values
    let css = generate_css("start-[10px]");
    assert!(css.contains("inset-inline-start:10px"), "Expected inset-inline-start:10px, got: {}", css);
    
    let css = generate_css("start-[2.5rem]");
    assert!(css.contains("inset-inline-start:2.5rem"), "Expected inset-inline-start:2.5rem, got: {}", css);
    
    let css = generate_css("start-[50%]");
    assert!(css.contains("inset-inline-start:50%"), "Expected inset-inline-start:50%, got: {}", css);
    
    let css = generate_css("-start-[10px]");
    assert!(css.contains("inset-inline-start:-10px"), "Expected inset-inline-start:-10px, got: {}", css);
}

#[test]
fn test_logical_position_end_numeric() {
    // Test numeric values
    let css = generate_css("end-0");
    assert!(css.contains("inset-inline-end:0rem"), "Expected inset-inline-end:0rem, got: {}", css);
    
    let css = generate_css("end-1");
    assert!(css.contains("inset-inline-end:0.25rem"), "Expected inset-inline-end:0.25rem, got: {}", css);
    
    let css = generate_css("end-2");
    assert!(css.contains("inset-inline-end:0.5rem"), "Expected inset-inline-end:0.5rem, got: {}", css);
    
    let css = generate_css("end-4");
    assert!(css.contains("inset-inline-end:1rem"), "Expected inset-inline-end:1rem, got: {}", css);
    
    let css = generate_css("end-8");
    assert!(css.contains("inset-inline-end:2rem"), "Expected inset-inline-end:2rem, got: {}", css);
    
    let css = generate_css("end-16");
    assert!(css.contains("inset-inline-end:4rem"), "Expected inset-inline-end:4rem, got: {}", css);
    
    let css = generate_css("end-32");
    assert!(css.contains("inset-inline-end:8rem"), "Expected inset-inline-end:8rem, got: {}", css);
    
    let css = generate_css("end-64");
    assert!(css.contains("inset-inline-end:16rem"), "Expected inset-inline-end:16rem, got: {}", css);
    
    let css = generate_css("end-96");
    assert!(css.contains("inset-inline-end:24rem"), "Expected inset-inline-end:24rem, got: {}", css);
}

#[test]
fn test_logical_position_end_fractional() {
    // Test fractional values
    let css = generate_css("end-0.5");
    assert!(css.contains("inset-inline-end:0.125rem"), "Expected inset-inline-end:0.125rem, got: {}", css);
    
    let css = generate_css("end-1.5");
    assert!(css.contains("inset-inline-end:0.375rem"), "Expected inset-inline-end:0.375rem, got: {}", css);
    
    let css = generate_css("end-2.5");
    assert!(css.contains("inset-inline-end:0.625rem"), "Expected inset-inline-end:0.625rem, got: {}", css);
}

#[test]
fn test_logical_position_end_percentage() {
    // Test percentage values
    let css = generate_css("end-1/2");
    assert!(css.contains("inset-inline-end:50%"), "Expected inset-inline-end:50%, got: {}", css);
    
    let css = generate_css("end-1/3");
    assert!(css.contains("inset-inline-end:33.333333%"), "Expected inset-inline-end:33.333333%, got: {}", css);
    
    let css = generate_css("end-2/3");
    assert!(css.contains("inset-inline-end:66.666667%"), "Expected inset-inline-end:66.666667%, got: {}", css);
    
    let css = generate_css("end-1/4");
    assert!(css.contains("inset-inline-end:25%"), "Expected inset-inline-end:25%, got: {}", css);
    
    let css = generate_css("end-2/4");
    assert!(css.contains("inset-inline-end:50%"), "Expected inset-inline-end:50%, got: {}", css);
    
    let css = generate_css("end-3/4");
    assert!(css.contains("inset-inline-end:75%"), "Expected inset-inline-end:75%, got: {}", css);
}

#[test]
fn test_logical_position_end_special() {
    // Test special values
    let css = generate_css("end-px");
    assert!(css.contains("inset-inline-end:1px"), "Expected inset-inline-end:1px, got: {}", css);
    
    let css = generate_css("end-full");
    assert!(css.contains("inset-inline-end:100%"), "Expected inset-inline-end:100%, got: {}", css);
    
    let css = generate_css("end-auto");
    assert!(css.contains("inset-inline-end:auto"), "Expected inset-inline-end:auto, got: {}", css);
}

#[test]
fn test_logical_position_end_negative() {
    // Test negative values
    let css = generate_css("-end-1");
    assert!(css.contains("inset-inline-end:-0.25rem"), "Expected inset-inline-end:-0.25rem, got: {}", css);
    
    let css = generate_css("-end-2");
    assert!(css.contains("inset-inline-end:-0.5rem"), "Expected inset-inline-end:-0.5rem, got: {}", css);
    
    let css = generate_css("-end-4");
    assert!(css.contains("inset-inline-end:-1rem"), "Expected inset-inline-end:-1rem, got: {}", css);
    
    let css = generate_css("-end-px");
    assert!(css.contains("inset-inline-end:-1px"), "Expected inset-inline-end:-1px, got: {}", css);
    
    let css = generate_css("-end-full");
    assert!(css.contains("inset-inline-end:-100%"), "Expected inset-inline-end:-100%, got: {}", css);
}

#[test]
fn test_logical_position_end_arbitrary() {
    // Test arbitrary values
    let css = generate_css("end-[10px]");
    assert!(css.contains("inset-inline-end:10px"), "Expected inset-inline-end:10px, got: {}", css);
    
    let css = generate_css("end-[2.5rem]");
    assert!(css.contains("inset-inline-end:2.5rem"), "Expected inset-inline-end:2.5rem, got: {}", css);
    
    let css = generate_css("end-[50%]");
    assert!(css.contains("inset-inline-end:50%"), "Expected inset-inline-end:50%, got: {}", css);
    
    let css = generate_css("-end-[10px]");
    assert!(css.contains("inset-inline-end:-10px"), "Expected inset-inline-end:-10px, got: {}", css);
}

#[test]
fn test_logical_position_combined() {
    // Test combining start and end
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("start-4 end-4", false);
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("inset-inline-start:1rem"), "Expected inset-inline-start:1rem, got: {}", css);
    assert!(css.contains("inset-inline-end:1rem"), "Expected inset-inline-end:1rem, got: {}", css);
    
    // Test with other position properties
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("top-0 start-0", false);
    let css = builder.bundle().unwrap_or_default();
    
    assert!(css.contains("top:0rem"), "Expected top:0rem, got: {}", css);
    assert!(css.contains("inset-inline-start:0rem"), "Expected inset-inline-start:0rem, got: {}", css);
}

#[test]
fn test_logical_position_responsive() {
    // Test responsive variants
    let css = generate_css("sm:start-4");
    assert!(css.contains("@media (min-width: 640px)"), "Expected media query, got: {}", css);
    assert!(css.contains("inset-inline-start:1rem"), "Expected inset-inline-start:1rem in media query, got: {}", css);
    
    let css = generate_css("md:end-8");
    assert!(css.contains("@media (min-width: 768px)"), "Expected media query, got: {}", css);
    assert!(css.contains("inset-inline-end:2rem"), "Expected inset-inline-end:2rem in media query, got: {}", css);
    
    let css = generate_css("lg:start-auto");
    assert!(css.contains("@media (min-width: 1024px)"), "Expected media query, got: {}", css);
    assert!(css.contains("inset-inline-start:auto"), "Expected inset-inline-start:auto in media query, got: {}", css);
}

#[test]
fn test_logical_position_hover_states() {
    // Test hover and other state variants
    let css = generate_css("hover:start-2");
    assert!(css.contains(":hover"), "Expected :hover pseudo-class, got: {}", css);
    assert!(css.contains("inset-inline-start:0.5rem"), "Expected inset-inline-start:0.5rem on hover, got: {}", css);
    
    let css = generate_css("focus:end-4");
    assert!(css.contains(":focus"), "Expected :focus pseudo-class, got: {}", css);
    assert!(css.contains("inset-inline-end:1rem"), "Expected inset-inline-end:1rem on focus, got: {}", css);
}