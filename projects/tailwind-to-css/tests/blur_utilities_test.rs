use tailwind_css::TailwindBuilder;

fn generate_css(class: &str) -> String {
    let mut builder = TailwindBuilder::default();
    
    // Process the class
    let _ = builder.trace(class, false);
    
    // Get the generated CSS
    builder.bundle().unwrap_or_default()
}

#[test]
fn test_blur_none() {
    let css = generate_css("blur-none");
    assert!(css.contains("--tw-blur:blur(0)"), "Expected --tw-blur:blur(0), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_sm() {
    let css = generate_css("blur-sm");
    assert!(css.contains("--tw-blur:blur(4px)"), "Expected --tw-blur:blur(4px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_default() {
    let css = generate_css("blur");
    assert!(css.contains("--tw-blur:blur(8px)"), "Expected --tw-blur:blur(8px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_md() {
    let css = generate_css("blur-md");
    assert!(css.contains("--tw-blur:blur(12px)"), "Expected --tw-blur:blur(12px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_lg() {
    let css = generate_css("blur-lg");
    assert!(css.contains("--tw-blur:blur(16px)"), "Expected --tw-blur:blur(16px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_xl() {
    let css = generate_css("blur-xl");
    assert!(css.contains("--tw-blur:blur(24px)"), "Expected --tw-blur:blur(24px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_2xl() {
    let css = generate_css("blur-2xl");
    assert!(css.contains("--tw-blur:blur(40px)"), "Expected --tw-blur:blur(40px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_3xl() {
    let css = generate_css("blur-3xl");
    assert!(css.contains("--tw-blur:blur(64px)"), "Expected --tw-blur:blur(64px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_arbitrary_value() {
    let css = generate_css("blur-[2px]");
    assert!(css.contains("--tw-blur:blur(2px)"), "Expected --tw-blur:blur(2px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_arbitrary_rem() {
    let css = generate_css("blur-[0.5rem]");
    assert!(css.contains("--tw-blur:blur(0.5rem)"), "Expected --tw-blur:blur(0.5rem), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_blur_numeric_value() {
    let css = generate_css("blur-4");
    assert!(css.contains("--tw-blur:blur(4px)"), "Expected --tw-blur:blur(4px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_blur_none() {
    let css = generate_css("backdrop-blur-none");
    assert!(css.contains("--tw-backdrop-blur:blur(0)"), "Expected --tw-backdrop-blur:blur(0), got: {}", css);
    assert!(css.contains("backdrop-filter:var(--tw-backdrop-blur)"), "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_blur_sm() {
    let css = generate_css("backdrop-blur-sm");
    assert!(css.contains("--tw-backdrop-blur:blur(4px)"), "Expected --tw-backdrop-blur:blur(4px), got: {}", css);
    assert!(css.contains("backdrop-filter:var(--tw-backdrop-blur)"), "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_blur_default() {
    let css = generate_css("backdrop-blur");
    assert!(css.contains("--tw-backdrop-blur:blur(8px)"), "Expected --tw-backdrop-blur:blur(8px), got: {}", css);
    assert!(css.contains("backdrop-filter:var(--tw-backdrop-blur)"), "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_blur_lg() {
    let css = generate_css("backdrop-blur-lg");
    assert!(css.contains("--tw-backdrop-blur:blur(16px)"), "Expected --tw-backdrop-blur:blur(16px), got: {}", css);
    assert!(css.contains("backdrop-filter:var(--tw-backdrop-blur)"), "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_blur_2xl() {
    let css = generate_css("backdrop-blur-2xl");
    assert!(css.contains("--tw-backdrop-blur:blur(40px)"), "Expected --tw-backdrop-blur:blur(40px), got: {}", css);
    assert!(css.contains("backdrop-filter:var(--tw-backdrop-blur)"), "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_blur_arbitrary() {
    let css = generate_css("backdrop-blur-[10px]");
    assert!(css.contains("--tw-backdrop-blur:blur(10px)"), "Expected --tw-backdrop-blur:blur(10px), got: {}", css);
    assert!(css.contains("backdrop-filter:var(--tw-backdrop-blur)"), "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_hover_blur() {
    let css = generate_css("hover:blur-lg");
    assert!(css.contains(":hover"), "Expected hover pseudo-class");
    assert!(css.contains("--tw-blur:blur(16px)"), "Expected --tw-blur:blur(16px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_group_hover_blur() {
    let css = generate_css("group-hover:blur-xl");
    // Check for the blur effect with CSS variables
    assert!(css.contains("--tw-blur:blur(24px)"), "Expected --tw-blur:blur(24px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
    // Verify it's a group-hover variant
    assert!(css.contains("group-hover") || css.contains(".group:hover"), "Expected group-hover variant in CSS, got: {}", css);
}

#[test]
fn test_peer_checked_blur() {
    let css = generate_css("peer-checked:blur-md");
    // Check for the blur effect with CSS variables
    assert!(css.contains("--tw-blur:blur(12px)"), "Expected --tw-blur:blur(12px), got: {}", css);
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable, got: {}", css);
    // Verify it's a peer-checked variant
    assert!(css.contains("peer-checked") || css.contains(".peer:checked"), "Expected peer-checked variant in CSS, got: {}", css);
}

#[test]
fn test_multiple_blur_classes() {
    let css = generate_css("blur-sm hover:blur-lg");
    assert!(css.contains("--tw-blur:blur(4px)"), "Expected --tw-blur:blur(4px) for blur-sm");
    assert!(css.contains("--tw-blur:blur(16px)"), "Expected --tw-blur:blur(16px) for hover:blur-lg");
    assert!(css.contains("filter:var(--tw-blur)"), "Expected filter with CSS variable");
}