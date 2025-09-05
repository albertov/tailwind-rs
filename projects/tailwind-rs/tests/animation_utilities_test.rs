use tailwind_rs::TailwindBuilder;

#[test]
fn test_animate_none() {
    let mut tw = TailwindBuilder::default();
    tw.trace("animate-none", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain the animation property
    assert!(css.contains("animation:none"));
    // Should not contain any keyframes for none
    assert!(!css.contains("@keyframes"));
}

#[test]
fn test_animate_spin() {
    let mut tw = TailwindBuilder::default();
    tw.trace("animate-spin", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain the animation property
    assert!(css.contains("animation:spin 1s linear infinite"));
    // Should contain the spin keyframes
    assert!(css.contains("@keyframes spin"));
    assert!(css.contains("transform: rotate(0deg)"));
    assert!(css.contains("transform: rotate(360deg)"));
}

#[test]
fn test_animate_ping() {
    let mut tw = TailwindBuilder::default();
    tw.trace("animate-ping", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain the animation property
    assert!(css.contains("animation:ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"));
    // Should contain the ping keyframes
    assert!(css.contains("@keyframes ping"));
    assert!(css.contains("transform:scale(2)"));
    assert!(css.contains("opacity:0"));
}

#[test]
fn test_animate_pulse() {
    let mut tw = TailwindBuilder::default();
    tw.trace("animate-pulse", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain the animation property
    assert!(css.contains("animation:pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"));
    // Should contain the pulse keyframes
    assert!(css.contains("@keyframes pulse"));
    assert!(css.contains("opacity:1"));
    assert!(css.contains("opacity:.5"));
}

#[test]
fn test_animate_bounce() {
    let mut tw = TailwindBuilder::default();
    tw.trace("animate-bounce", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain the animation property
    assert!(css.contains("animation:bounce 1s infinite"));
    // Should contain the bounce keyframes
    assert!(css.contains("@keyframes bounce"));
    assert!(css.contains("transform:translateY(-25%)"));
    assert!(css.contains("transform:translateY(0)"));
    assert!(css.contains("cubic-bezier(0.8,0,1,1)"));
    assert!(css.contains("cubic-bezier(0,0,0.2,1)"));
}

#[test]
fn test_animate_with_hover_variant() {
    let mut tw = TailwindBuilder::default();
    tw.trace("hover:animate-spin", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain hover pseudo-class
    assert!(css.contains(":hover"));
    assert!(css.contains("animation:spin 1s linear infinite"));
    assert!(css.contains("@keyframes spin"));
}

#[test]
fn test_animate_with_responsive_variant() {
    let mut tw = TailwindBuilder::default();
    tw.trace("lg:animate-pulse", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain media query
    assert!(css.contains("@media"));
    assert!(css.contains("animation:pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"));
    assert!(css.contains("@keyframes pulse"));
}

#[test]
fn test_animate_with_dark_variant() {
    let mut tw = TailwindBuilder::default();
    tw.trace("dark:animate-bounce", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain dark mode selector
    assert!(css.contains(".dark"));
    assert!(css.contains("animation:bounce 1s infinite"));
    assert!(css.contains("@keyframes bounce"));
}

#[test]
fn test_animate_with_group_hover_variant() {
    let mut tw = TailwindBuilder::default();
    tw.trace("group-hover:animate-ping", false).unwrap();
    let css = tw.bundle().unwrap();
    
    println!("group-hover:animate-ping CSS output:\n{}", css);
    
    // Should contain group-hover selector
    assert!(css.contains(":hover"));
    assert!(css.contains("animation:ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"));
    assert!(css.contains("@keyframes ping"));
}

#[test]
fn test_multiple_animations() {
    let mut tw = TailwindBuilder::default();
    tw.trace("animate-spin", false).unwrap();
    tw.trace("animate-bounce", false).unwrap();
    tw.trace("animate-pulse", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Should contain all three animations
    assert!(css.contains("@keyframes spin"));
    assert!(css.contains("@keyframes bounce"));
    assert!(css.contains("@keyframes pulse"));
    
    // Each should have its own class
    assert!(css.contains(".animate-spin"));
    assert!(css.contains(".animate-bounce"));
    assert!(css.contains(".animate-pulse"));
}

#[test]
fn test_arbitrary_animation() {
    let mut tw = TailwindBuilder::default();
    tw.trace("animate-[wiggle_1s_ease-in-out_infinite]", false).unwrap();
    let css = tw.bundle().unwrap();
    
    println!("arbitrary animation CSS output:\n{}", css);
    
    // Should contain the arbitrary animation value (underscores converted to spaces)
    assert!(css.contains("animation:wiggle 1s ease-in-out infinite"));
}

#[test]
fn test_animation_class_selectors() {
    let mut tw = TailwindBuilder::default();
    tw.trace("animate-spin", false).unwrap();
    tw.trace("animate-ping", false).unwrap();
    tw.trace("animate-pulse", false).unwrap();
    tw.trace("animate-bounce", false).unwrap();
    tw.trace("animate-none", false).unwrap();
    let css = tw.bundle().unwrap();
    
    println!("multiple animations CSS output:\n{}", css);
    
    // Verify class selectors are present (without assuming exact format)
    assert!(css.contains("animate-spin"));
    assert!(css.contains("animate-ping"));
    assert!(css.contains("animate-pulse"));
    assert!(css.contains("animate-bounce"));
    assert!(css.contains("animate-none"));
}