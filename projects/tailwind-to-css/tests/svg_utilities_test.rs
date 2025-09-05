use tailwind_css::TailwindBuilder;

fn generate_css(class: &str) -> String {
    let mut builder = TailwindBuilder::default();
    
    // Process the class
    let _ = builder.trace(class, false);
    
    // Get the generated CSS
    builder.bundle().unwrap_or_default()
}

// ===== Stroke Dasharray Tests =====

#[test]
fn test_stroke_dash_0() {
    let css = generate_css("stroke-dash-0");
    assert!(css.contains("stroke-dasharray:0"), "Expected stroke-dasharray:0, got: {}", css);
}

#[test]
fn test_stroke_dash_1() {
    let css = generate_css("stroke-dash-1");
    assert!(css.contains("stroke-dasharray:1"), "Expected stroke-dasharray:1, got: {}", css);
}

#[test]
fn test_stroke_dash_5() {
    let css = generate_css("stroke-dash-5");
    assert!(css.contains("stroke-dasharray:5"), "Expected stroke-dasharray:5, got: {}", css);
}

#[test]
fn test_stroke_dash_10() {
    let css = generate_css("stroke-dash-10");
    assert!(css.contains("stroke-dasharray:10"), "Expected stroke-dasharray:10, got: {}", css);
}

#[test]
fn test_stroke_dash_arbitrary() {
    let css = generate_css("stroke-dash-[15]");
    assert!(css.contains("stroke-dasharray:15"), "Expected stroke-dasharray:15, got: {}", css);
}

#[test]
fn test_stroke_dash_arbitrary_complex() {
    let css = generate_css("stroke-dash-[5_10]");
    assert!(css.contains("stroke-dasharray:5 10"), "Expected stroke-dasharray:5 10, got: {}", css);
}

// ===== Stroke Dashoffset Tests =====

#[test]
fn test_stroke_offset_0() {
    let css = generate_css("stroke-offset-0");
    assert!(css.contains("stroke-dashoffset:0"), "Expected stroke-dashoffset:0, got: {}", css);
}

#[test]
fn test_stroke_offset_1() {
    let css = generate_css("stroke-offset-1");
    assert!(css.contains("stroke-dashoffset:1"), "Expected stroke-dashoffset:1, got: {}", css);
}

#[test]
fn test_stroke_offset_5() {
    let css = generate_css("stroke-offset-5");
    assert!(css.contains("stroke-dashoffset:5"), "Expected stroke-dashoffset:5, got: {}", css);
}

#[test]
fn test_stroke_offset_10() {
    let css = generate_css("stroke-offset-10");
    assert!(css.contains("stroke-dashoffset:10"), "Expected stroke-dashoffset:10, got: {}", css);
}

#[test]
fn test_stroke_offset_arbitrary() {
    let css = generate_css("stroke-offset-[15]");
    assert!(css.contains("stroke-dashoffset:15"), "Expected stroke-dashoffset:15, got: {}", css);
}

// ===== Fill Opacity Tests =====

#[test]
fn test_fill_opacity_0() {
    let css = generate_css("fill-opacity-0");
    assert!(css.contains("fill-opacity:0"), "Expected fill-opacity:0, got: {}", css);
}

#[test]
fn test_fill_opacity_5() {
    let css = generate_css("fill-opacity-5");
    assert!(css.contains("fill-opacity:0.05"), "Expected fill-opacity:0.05, got: {}", css);
}

#[test]
fn test_fill_opacity_10() {
    let css = generate_css("fill-opacity-10");
    assert!(css.contains("fill-opacity:0.1"), "Expected fill-opacity:0.1, got: {}", css);
}

#[test]
fn test_fill_opacity_20() {
    let css = generate_css("fill-opacity-20");
    assert!(css.contains("fill-opacity:0.2"), "Expected fill-opacity:0.2, got: {}", css);
}

#[test]
fn test_fill_opacity_25() {
    let css = generate_css("fill-opacity-25");
    assert!(css.contains("fill-opacity:0.25"), "Expected fill-opacity:0.25, got: {}", css);
}

#[test]
fn test_fill_opacity_30() {
    let css = generate_css("fill-opacity-30");
    assert!(css.contains("fill-opacity:0.3"), "Expected fill-opacity:0.3, got: {}", css);
}

#[test]
fn test_fill_opacity_40() {
    let css = generate_css("fill-opacity-40");
    assert!(css.contains("fill-opacity:0.4"), "Expected fill-opacity:0.4, got: {}", css);
}

#[test]
fn test_fill_opacity_50() {
    let css = generate_css("fill-opacity-50");
    assert!(css.contains("fill-opacity:0.5"), "Expected fill-opacity:0.5, got: {}", css);
}

#[test]
fn test_fill_opacity_60() {
    let css = generate_css("fill-opacity-60");
    assert!(css.contains("fill-opacity:0.6"), "Expected fill-opacity:0.6, got: {}", css);
}

#[test]
fn test_fill_opacity_70() {
    let css = generate_css("fill-opacity-70");
    assert!(css.contains("fill-opacity:0.7"), "Expected fill-opacity:0.7, got: {}", css);
}

#[test]
fn test_fill_opacity_75() {
    let css = generate_css("fill-opacity-75");
    assert!(css.contains("fill-opacity:0.75"), "Expected fill-opacity:0.75, got: {}", css);
}

#[test]
fn test_fill_opacity_80() {
    let css = generate_css("fill-opacity-80");
    assert!(css.contains("fill-opacity:0.8"), "Expected fill-opacity:0.8, got: {}", css);
}

#[test]
fn test_fill_opacity_90() {
    let css = generate_css("fill-opacity-90");
    assert!(css.contains("fill-opacity:0.9"), "Expected fill-opacity:0.9, got: {}", css);
}

#[test]
fn test_fill_opacity_95() {
    let css = generate_css("fill-opacity-95");
    assert!(css.contains("fill-opacity:0.95"), "Expected fill-opacity:0.95, got: {}", css);
}

#[test]
fn test_fill_opacity_100() {
    let css = generate_css("fill-opacity-100");
    assert!(css.contains("fill-opacity:1"), "Expected fill-opacity:1, got: {}", css);
}

#[test]
fn test_fill_opacity_arbitrary() {
    let css = generate_css("fill-opacity-[0.33]");
    assert!(css.contains("fill-opacity:0.33"), "Expected fill-opacity:0.33, got: {}", css);
}

// ===== Stroke Opacity Tests =====

#[test]
fn test_stroke_opacity_0() {
    let css = generate_css("stroke-opacity-0");
    assert!(css.contains("stroke-opacity:0"), "Expected stroke-opacity:0, got: {}", css);
}

#[test]
fn test_stroke_opacity_5() {
    let css = generate_css("stroke-opacity-5");
    assert!(css.contains("stroke-opacity:0.05"), "Expected stroke-opacity:0.05, got: {}", css);
}

#[test]
fn test_stroke_opacity_10() {
    let css = generate_css("stroke-opacity-10");
    assert!(css.contains("stroke-opacity:0.1"), "Expected stroke-opacity:0.1, got: {}", css);
}

#[test]
fn test_stroke_opacity_20() {
    let css = generate_css("stroke-opacity-20");
    assert!(css.contains("stroke-opacity:0.2"), "Expected stroke-opacity:0.2, got: {}", css);
}

#[test]
fn test_stroke_opacity_25() {
    let css = generate_css("stroke-opacity-25");
    assert!(css.contains("stroke-opacity:0.25"), "Expected stroke-opacity:0.25, got: {}", css);
}

#[test]
fn test_stroke_opacity_30() {
    let css = generate_css("stroke-opacity-30");
    assert!(css.contains("stroke-opacity:0.3"), "Expected stroke-opacity:0.3, got: {}", css);
}

#[test]
fn test_stroke_opacity_40() {
    let css = generate_css("stroke-opacity-40");
    assert!(css.contains("stroke-opacity:0.4"), "Expected stroke-opacity:0.4, got: {}", css);
}

#[test]
fn test_stroke_opacity_50() {
    let css = generate_css("stroke-opacity-50");
    assert!(css.contains("stroke-opacity:0.5"), "Expected stroke-opacity:0.5, got: {}", css);
}

#[test]
fn test_stroke_opacity_60() {
    let css = generate_css("stroke-opacity-60");
    assert!(css.contains("stroke-opacity:0.6"), "Expected stroke-opacity:0.6, got: {}", css);
}

#[test]
fn test_stroke_opacity_70() {
    let css = generate_css("stroke-opacity-70");
    assert!(css.contains("stroke-opacity:0.7"), "Expected stroke-opacity:0.7, got: {}", css);
}

#[test]
fn test_stroke_opacity_75() {
    let css = generate_css("stroke-opacity-75");
    assert!(css.contains("stroke-opacity:0.75"), "Expected stroke-opacity:0.75, got: {}", css);
}

#[test]
fn test_stroke_opacity_80() {
    let css = generate_css("stroke-opacity-80");
    assert!(css.contains("stroke-opacity:0.8"), "Expected stroke-opacity:0.8, got: {}", css);
}

#[test]
fn test_stroke_opacity_90() {
    let css = generate_css("stroke-opacity-90");
    assert!(css.contains("stroke-opacity:0.9"), "Expected stroke-opacity:0.9, got: {}", css);
}

#[test]
fn test_stroke_opacity_95() {
    let css = generate_css("stroke-opacity-95");
    assert!(css.contains("stroke-opacity:0.95"), "Expected stroke-opacity:0.95, got: {}", css);
}

#[test]
fn test_stroke_opacity_100() {
    let css = generate_css("stroke-opacity-100");
    assert!(css.contains("stroke-opacity:1"), "Expected stroke-opacity:1, got: {}", css);
}

#[test]
fn test_stroke_opacity_arbitrary() {
    let css = generate_css("stroke-opacity-[0.67]");
    assert!(css.contains("stroke-opacity:0.67"), "Expected stroke-opacity:0.67, got: {}", css);
}

// ===== Stroke Linecap Tests =====

#[test]
fn test_stroke_cap_butt() {
    let css = generate_css("stroke-cap-butt");
    assert!(css.contains("stroke-linecap:butt"), "Expected stroke-linecap:butt, got: {}", css);
}

#[test]
fn test_stroke_cap_round() {
    let css = generate_css("stroke-cap-round");
    assert!(css.contains("stroke-linecap:round"), "Expected stroke-linecap:round, got: {}", css);
}

#[test]
fn test_stroke_cap_square() {
    let css = generate_css("stroke-cap-square");
    assert!(css.contains("stroke-linecap:square"), "Expected stroke-linecap:square, got: {}", css);
}

// ===== Stroke Linejoin Tests =====

#[test]
fn test_stroke_join_miter() {
    let css = generate_css("stroke-join-miter");
    assert!(css.contains("stroke-linejoin:miter"), "Expected stroke-linejoin:miter, got: {}", css);
}

#[test]
fn test_stroke_join_round() {
    let css = generate_css("stroke-join-round");
    assert!(css.contains("stroke-linejoin:round"), "Expected stroke-linejoin:round, got: {}", css);
}

#[test]
fn test_stroke_join_bevel() {
    let css = generate_css("stroke-join-bevel");
    assert!(css.contains("stroke-linejoin:bevel"), "Expected stroke-linejoin:bevel, got: {}", css);
}

// ===== Combined Tests with Modifiers =====

#[test]
fn test_fill_opacity_hover() {
    let css = generate_css("hover:fill-opacity-50");
    assert!(css.contains(":hover"), "Expected hover selector, got: {}", css);
    assert!(css.contains("fill-opacity:0.5"), "Expected fill-opacity:0.5, got: {}", css);
}

#[test]
fn test_stroke_opacity_responsive() {
    let css = generate_css("md:stroke-opacity-75");
    assert!(css.contains("@media"), "Expected media query, got: {}", css);
    assert!(css.contains("stroke-opacity:0.75"), "Expected stroke-opacity:0.75, got: {}", css);
}

#[test]
fn test_stroke_cap_hover() {
    let css = generate_css("hover:stroke-cap-round");
    assert!(css.contains(":hover"), "Expected hover selector, got: {}", css);
    assert!(css.contains("stroke-linecap:round"), "Expected stroke-linecap:round, got: {}", css);
}

#[test]
fn test_stroke_join_focus() {
    let css = generate_css("focus:stroke-join-bevel");
    assert!(css.contains(":focus"), "Expected focus selector, got: {}", css);
    assert!(css.contains("stroke-linejoin:bevel"), "Expected stroke-linejoin:bevel, got: {}", css);
}

#[test]
fn test_stroke_dash_responsive() {
    let css = generate_css("lg:stroke-dash-5");
    assert!(css.contains("@media"), "Expected media query, got: {}", css);
    assert!(css.contains("stroke-dasharray:5"), "Expected stroke-dasharray:5, got: {}", css);
}

#[test]
fn test_stroke_offset_active() {
    let css = generate_css("active:stroke-offset-3");
    assert!(css.contains(":active"), "Expected active selector, got: {}", css);
    assert!(css.contains("stroke-dashoffset:3"), "Expected stroke-dashoffset:3, got: {}", css);
}