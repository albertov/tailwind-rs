use tailwind_css::TailwindBuilder;

fn generate_css(class: &str) -> String {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace(class, false);
    builder.bundle().unwrap_or_default()
}

// === REGULAR FILTER TESTS ===

#[test]
fn test_brightness_utilities() {
    let css = generate_css("brightness-50");
    assert!(css.contains("--tw-brightness:brightness(50%)") || css.contains("--tw-brightness:brightness(0.5)"), 
            "Expected brightness value, got: {}", css);
    assert!(css.contains("filter:") && css.contains("var(--tw-brightness)"), 
            "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_contrast_utilities() {
    let css = generate_css("contrast-125");
    assert!(css.contains("--tw-contrast:contrast(125%)") || css.contains("--tw-contrast:contrast(1.25)"), 
            "Expected contrast value, got: {}", css);
    assert!(css.contains("filter:") && css.contains("var(--tw-contrast)"), 
            "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_grayscale_utilities() {
    let css = generate_css("grayscale");
    assert!(css.contains("--tw-grayscale:grayscale(100%)") || css.contains("--tw-grayscale:grayscale(1)"), 
            "Expected grayscale value, got: {}", css);
    assert!(css.contains("filter:") && css.contains("var(--tw-grayscale)"), 
            "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_hue_rotate_utilities() {
    let css = generate_css("hue-rotate-90");
    assert!(css.contains("--tw-hue-rotate:hue-rotate(90deg)"), 
            "Expected hue-rotate value, got: {}", css);
    assert!(css.contains("filter:") && css.contains("var(--tw-hue-rotate)"), 
            "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_negative_hue_rotate() {
    let css = generate_css("-hue-rotate-45");
    assert!(css.contains("--tw-hue-rotate:hue-rotate(-45deg)"), 
            "Expected negative hue-rotate value, got: {}", css);
}

#[test]
fn test_invert_utilities() {
    let css = generate_css("invert");
    assert!(css.contains("--tw-invert:invert(100%)") || css.contains("--tw-invert:invert(1)"), 
            "Expected invert value, got: {}", css);
    assert!(css.contains("filter:") && css.contains("var(--tw-invert)"), 
            "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_saturate_utilities() {
    let css = generate_css("saturate-150");
    assert!(css.contains("--tw-saturate:saturate(150%)") || css.contains("--tw-saturate:saturate(1.5)"), 
            "Expected saturate value, got: {}", css);
    assert!(css.contains("filter:") && css.contains("var(--tw-saturate)"), 
            "Expected filter with CSS variable, got: {}", css);
}

#[test]
fn test_sepia_utilities() {
    let css = generate_css("sepia");
    assert!(css.contains("--tw-sepia:sepia(100%)") || css.contains("--tw-sepia:sepia(1)"), 
            "Expected sepia value, got: {}", css);
    assert!(css.contains("filter:") && css.contains("var(--tw-sepia)"), 
            "Expected filter with CSS variable, got: {}", css);
}

// === DROP-SHADOW TESTS ===

#[test]
fn test_drop_shadow_default() {
    let css = generate_css("drop-shadow");
    assert!(css.contains("filter:drop-shadow(0 1px 2px rgb(0 0 0 / 0.15))"), 
            "Expected default drop-shadow value, got: {}", css);
}

#[test]
fn test_drop_shadow_sm() {
    let css = generate_css("drop-shadow-sm");
    assert!(css.contains("filter:drop-shadow(0 1px 2px rgb(0 0 0 / 0.15))"), 
            "Expected drop-shadow-sm value, got: {}", css);
}

#[test]
fn test_drop_shadow_md() {
    let css = generate_css("drop-shadow-md");
    assert!(css.contains("filter:drop-shadow(0 3px 3px rgb(0 0 0 / 0.12))"), 
            "Expected drop-shadow-md value, got: {}", css);
}

#[test]
fn test_drop_shadow_lg() {
    let css = generate_css("drop-shadow-lg");
    assert!(css.contains("filter:drop-shadow(0 4px 4px rgb(0 0 0 / 0.15))"), 
            "Expected drop-shadow-lg value, got: {}", css);
}

#[test]
fn test_drop_shadow_xl() {
    let css = generate_css("drop-shadow-xl");
    assert!(css.contains("filter:drop-shadow(0 9px 7px rgb(0 0 0 / 0.1))"), 
            "Expected drop-shadow-xl value, got: {}", css);
}

#[test]
fn test_drop_shadow_2xl() {
    let css = generate_css("drop-shadow-2xl");
    assert!(css.contains("filter:drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))"), 
            "Expected drop-shadow-2xl value, got: {}", css);
}

#[test]
fn test_drop_shadow_none() {
    let css = generate_css("drop-shadow-none");
    assert!(css.contains("filter:drop-shadow(0 0 #0000)"), 
            "Expected drop-shadow-none value, got: {}", css);
}

#[test]
fn test_drop_shadow_arbitrary() {
    let css = generate_css("drop-shadow-[0_2px_4px_rgba(0,0,0,0.1)]");
    assert!(css.contains("filter:0 2px 4px rgba(0,0,0,0.1)"), 
            "Expected arbitrary drop-shadow value, got: {}", css);
}

// === BACKDROP FILTER TESTS ===

#[test]
fn test_backdrop_blur() {
    let css = generate_css("backdrop-blur");
    assert!(css.contains("--tw-backdrop-blur:blur(8px)"), 
            "Expected backdrop-blur value, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-blur)"), 
            "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_brightness() {
    let css = generate_css("backdrop-brightness-75");
    assert!(css.contains("--tw-backdrop-brightness:brightness(75%)") || css.contains("--tw-backdrop-brightness:brightness(0.75)"), 
            "Expected backdrop-brightness value, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-brightness)"), 
            "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_contrast() {
    let css = generate_css("backdrop-contrast-150");
    assert!(css.contains("--tw-backdrop-contrast:contrast(150%)") || css.contains("--tw-backdrop-contrast:contrast(1.5)"), 
            "Expected backdrop-contrast value, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-contrast)"), 
            "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_grayscale() {
    let css = generate_css("backdrop-grayscale");
    assert!(css.contains("--tw-backdrop-grayscale:grayscale(100%)") || css.contains("--tw-backdrop-grayscale:grayscale(1)"), 
            "Expected backdrop-grayscale value, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-grayscale)"), 
            "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_hue_rotate() {
    let css = generate_css("backdrop-hue-rotate-60");
    assert!(css.contains("--tw-backdrop-hue-rotate:hue-rotate(60deg)"), 
            "Expected backdrop-hue-rotate value, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-hue-rotate)"), 
            "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_invert() {
    let css = generate_css("backdrop-invert");
    assert!(css.contains("--tw-backdrop-invert:invert(100%)") || css.contains("--tw-backdrop-invert:invert(1)"), 
            "Expected backdrop-invert value, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-invert)"), 
            "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_opacity() {
    let css = generate_css("backdrop-opacity-50");
    assert!(css.contains("backdrop-filter:opacity(50%)"), 
            "Expected backdrop-opacity value, got: {}", css);
}

#[test]
fn test_backdrop_saturate() {
    let css = generate_css("backdrop-saturate-200");
    assert!(css.contains("--tw-backdrop-saturate:saturate(200%)") || css.contains("--tw-backdrop-saturate:saturate(2)"), 
            "Expected backdrop-saturate value, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-saturate)"), 
            "Expected backdrop-filter with CSS variable, got: {}", css);
}

#[test]
fn test_backdrop_sepia() {
    let css = generate_css("backdrop-sepia");
    assert!(css.contains("--tw-backdrop-sepia:sepia(100%)") || css.contains("--tw-backdrop-sepia:sepia(1)"), 
            "Expected backdrop-sepia value, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-sepia)"), 
            "Expected backdrop-filter with CSS variable, got: {}", css);
}

// === FILTER COMBINATION TESTS ===

#[test]
fn test_multiple_filters() {
    let css = generate_css("blur-sm brightness-75 contrast-125");
    
    // Check that all filter variables are set
    assert!(css.contains("--tw-blur:blur(4px)"), 
            "Expected blur value in combined filters, got: {}", css);
    assert!(css.contains("--tw-brightness:brightness(75%)") || css.contains("--tw-brightness:brightness(0.75)"), 
            "Expected brightness value in combined filters, got: {}", css);
    assert!(css.contains("--tw-contrast:contrast(125%)") || css.contains("--tw-contrast:contrast(1.25)"), 
            "Expected contrast value in combined filters, got: {}", css);
    
    // Check that filter property uses all variables
    assert!(css.contains("filter:") && css.contains("var(--tw-blur)"), 
            "Expected filter with blur variable, got: {}", css);
    assert!(css.contains("filter:") && css.contains("var(--tw-brightness)"), 
            "Expected filter with brightness variable, got: {}", css);
    assert!(css.contains("filter:") && css.contains("var(--tw-contrast)"), 
            "Expected filter with contrast variable, got: {}", css);
}

#[test]
fn test_backdrop_filter_combination() {
    let css = generate_css("backdrop-blur-md backdrop-brightness-50 backdrop-saturate-150");
    
    // Check that all backdrop filter variables are set
    assert!(css.contains("--tw-backdrop-blur:blur(12px)"), 
            "Expected backdrop-blur value in combined filters, got: {}", css);
    assert!(css.contains("--tw-backdrop-brightness:brightness(50%)") || css.contains("--tw-backdrop-brightness:brightness(0.5)"), 
            "Expected backdrop-brightness value in combined filters, got: {}", css);
    assert!(css.contains("--tw-backdrop-saturate:saturate(150%)") || css.contains("--tw-backdrop-saturate:saturate(1.5)"), 
            "Expected backdrop-saturate value in combined filters, got: {}", css);
    
    // Check that backdrop-filter property uses all variables
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-blur)"), 
            "Expected backdrop-filter with blur variable, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-brightness)"), 
            "Expected backdrop-filter with brightness variable, got: {}", css);
    assert!(css.contains("backdrop-filter:") && css.contains("var(--tw-backdrop-saturate)"), 
            "Expected backdrop-filter with saturate variable, got: {}", css);
}

// === ARBITRARY VALUE TESTS ===

#[test]
fn test_brightness_arbitrary() {
    let css = generate_css("brightness-[1.75]");
    assert!(css.contains("--tw-brightness:brightness(1.75)"), 
            "Expected arbitrary brightness value, got: {}", css);
}

#[test]
fn test_contrast_arbitrary() {
    let css = generate_css("contrast-[2.5]");
    assert!(css.contains("--tw-contrast:contrast(2.5)"), 
            "Expected arbitrary contrast value, got: {}", css);
}

#[test]
fn test_hue_rotate_arbitrary() {
    let css = generate_css("hue-rotate-[270deg]");
    assert!(css.contains("--tw-hue-rotate:hue-rotate(270deg)"), 
            "Expected arbitrary hue-rotate value, got: {}", css);
}

#[test]
fn test_saturate_arbitrary() {
    let css = generate_css("saturate-[3]");
    assert!(css.contains("--tw-saturate:saturate(3)"), 
            "Expected arbitrary saturate value, got: {}", css);
}

#[test]
fn test_backdrop_blur_arbitrary() {
    let css = generate_css("backdrop-blur-[2px]");
    assert!(css.contains("--tw-backdrop-blur:blur(2px)"), 
            "Expected arbitrary backdrop-blur value, got: {}", css);
}

#[test]
fn test_backdrop_brightness_arbitrary() {
    let css = generate_css("backdrop-brightness-[0.25]");
    assert!(css.contains("--tw-backdrop-brightness:brightness(0.25)"), 
            "Expected arbitrary backdrop-brightness value, got: {}", css);
}

// === RESPONSIVE AND STATE VARIANTS ===

#[test]
fn test_hover_filter() {
    let css = generate_css("hover:brightness-110");
    assert!(css.contains(":hover"), "Expected hover pseudo-class");
    assert!(css.contains("--tw-brightness:brightness(110%)") || css.contains("--tw-brightness:brightness(1.1)"), 
            "Expected brightness value on hover, got: {}", css);
}

#[test]
fn test_responsive_filter() {
    let css = generate_css("md:blur-lg");
    assert!(css.contains("@media"), "Expected media query");
    assert!(css.contains("--tw-blur:blur(16px)"), 
            "Expected blur value in media query, got: {}", css);
}

#[test]
fn test_dark_mode_filter() {
    let css = generate_css("dark:invert");
    assert!(css.contains("--tw-invert:invert(100%)") || css.contains("--tw-invert:invert(1)"), 
            "Expected invert value in dark mode, got: {}", css);
}

#[test]
fn test_group_hover_backdrop() {
    let css = generate_css("group-hover:backdrop-blur-sm");
    assert!(css.contains("--tw-backdrop-blur:blur(4px)"), 
            "Expected backdrop-blur value on group hover, got: {}", css);
    assert!(css.contains("group-hover") || css.contains(".group:hover"), 
            "Expected group-hover variant, got: {}", css);
}