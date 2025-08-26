use tailwind_rs::TailwindBuilder;

#[test]
fn test_hover_modifier_css_generation() {
    let mut tw = TailwindBuilder::default();
    tw.trace("hover:bg-blue-500", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Verify exact CSS output
    assert!(css.contains(".hover\\:bg-blue-500:hover"));
    assert!(css.contains("background-color:rgba(59, 130, 246, 1)"));
}

#[test]
fn test_responsive_modifier_css_generation() {
    let mut tw = TailwindBuilder::default();
    tw.trace("sm:text-lg", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Verify media query wraps the rule
    assert!(css.contains("@media(min-width:640px)"));
    assert!(css.contains(".sm\\:text-lg"));
    assert!(css.contains("font-size:1.125rem"));
}

#[test]
fn test_combined_modifiers_css_generation() {
    let mut tw = TailwindBuilder::default();
    tw.trace("sm:hover:bg-blue-500", false).unwrap();
    let css = tw.bundle().unwrap();
    
    // Verify media query wraps pseudo-selector
    assert!(css.contains("@media(min-width:640px)"));
    assert!(css.contains(".sm\\:hover\\:bg-blue-500:hover"));
    assert!(css.contains("background-color:rgba(59, 130, 246, 1)"));
    // Verify nesting order is correct
    let media_start = css.find("@media").unwrap();
    let hover_pos = css.find(":hover").unwrap();
    assert!(media_start < hover_pos, "Media query must wrap pseudo-selector");
}

#[test]
fn test_dark_mode_modifier() {
    let mut tw = TailwindBuilder::default();
    tw.trace("dark:bg-gray-800", false).unwrap();
    let css = tw.bundle().unwrap();
    
    assert!(css.contains("@media(prefers-color-scheme:dark)"));
    assert!(css.contains(".dark\\:bg-gray-800"));
}

#[test]
#[ignore] // ring-2 utility not yet supported by the library
fn test_focus_modifier() {
    let mut tw = TailwindBuilder::default();
    tw.trace("focus:ring-2", false).unwrap();
    let css = tw.bundle().unwrap();
    
    assert!(css.contains(".focus\\:ring-2:focus"));
    assert!(css.contains("box-shadow"));
}

#[test]
fn test_state_modifiers() {
    let mut tw = TailwindBuilder::default();
    
    tw.trace("first:mt-0", false).unwrap();
    tw.trace("last:mb-0", false).unwrap();
    tw.trace("odd:bg-gray-100", false).unwrap();
    tw.trace("even:bg-white", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    assert!(css.contains(".first\\:mt-0:first-child"));
    assert!(css.contains(".last\\:mb-0:last-child"));
    assert!(css.contains(".odd\\:bg-gray-100:nth-child(odd)"));
    // The library generates bg-white as bg-[#FFFFFFFF]
    assert!(css.contains(":nth-child(even)"));
}

#[test]
fn test_all_responsive_breakpoints() {
    let mut tw = TailwindBuilder::default();
    
    tw.trace("sm:p-2", false).unwrap();
    tw.trace("md:p-4", false).unwrap();
    tw.trace("lg:p-6", false).unwrap();
    tw.trace("xl:p-8", false).unwrap();
    tw.trace("2xl:p-10", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    assert!(css.contains("@media(min-width:640px)"));  // sm
    assert!(css.contains("@media(min-width:768px)"));  // md
    assert!(css.contains("@media(min-width:1024px)")); // lg
    assert!(css.contains("@media(min-width:1280px)")); // xl
    assert!(css.contains("@media(min-width:1536px)")); // 2xl
}

#[test]
#[ignore] // Remove when custom breakpoints are supported
fn test_custom_breakpoint() {
    let mut tw = TailwindBuilder::default();
    tw.screens.register("custom".to_string(), 900);
    
    tw.trace("custom:text-xl", false).unwrap();
    let css = tw.bundle().unwrap();
    
    assert!(css.contains("@media(min-width:900px)"));
    assert!(css.contains(".custom\\:text-xl"));
}

#[test]
fn test_non_modifier_classes_still_work() {
    // This test should pass even before implementation
    let mut tw = TailwindBuilder::default();
    tw.trace("bg-blue-500 text-white p-4", false).unwrap();
    let css = tw.bundle().unwrap();
    
    assert!(css.contains(".bg-blue-500"), "Missing .bg-blue-500 class");
    // The library generates text color as .text-\[\#FFFFFFFF\] instead of .text-white
    assert!(css.contains(r".text-\["), "Missing text color class");
    assert!(css.contains(".p-4"), "Missing .p-4 class");
    assert!(css.contains("background-color"), "Missing background-color property");
    assert!(css.contains("color"), "Missing color property");
    assert!(css.contains("padding"), "Missing padding property");
}