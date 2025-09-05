use tailwind_css::TailwindBuilder;

fn generate_css(class: &str) -> String {
    let mut builder = TailwindBuilder::default();
    
    // Process the class
    let _ = builder.trace(class, false);
    
    // Get the generated CSS
    builder.bundle().unwrap_or_default()
}

#[test]
fn test_background_position_basic() {
    // Basic positions
    let css = generate_css("bg-top");
    assert!(css.contains("background-position:50% 0%"), "Expected background-position:50% 0%, got: {}", css);
    
    let css = generate_css("bg-bottom");
    assert!(css.contains("background-position:50% 100%"), "Expected background-position:50% 100%, got: {}", css);
    
    let css = generate_css("bg-center");
    assert!(css.contains("background-position:50% 50%"), "Expected background-position:50% 50%, got: {}", css);
    
    let css = generate_css("bg-left");
    assert!(css.contains("background-position:0% 50%"), "Expected background-position:0% 50%, got: {}", css);
    
    let css = generate_css("bg-right");
    assert!(css.contains("background-position:100% 50%"), "Expected background-position:100% 50%, got: {}", css);
}

#[test]
fn test_background_position_corners() {
    // Corner positions
    let css = generate_css("bg-left-top");
    assert!(css.contains("background-position:0% 0%"), "Expected background-position:0% 0%, got: {}", css);
    
    let css = generate_css("bg-left-bottom");
    assert!(css.contains("background-position:0% 100%"), "Expected background-position:0% 100%, got: {}", css);
    
    let css = generate_css("bg-right-top");
    assert!(css.contains("background-position:100% 0%"), "Expected background-position:100% 0%, got: {}", css);
    
    let css = generate_css("bg-right-bottom");
    assert!(css.contains("background-position:100% 100%"), "Expected background-position:100% 100%, got: {}", css);
}

#[test]
fn test_background_size_variants() {
    // Basic size utilities
    let css = generate_css("bg-auto");
    assert!(css.contains("background-size:auto"), "Expected background-size:auto, got: {}", css);
    
    let css = generate_css("bg-cover");
    assert!(css.contains("background-size:cover"), "Expected background-size:cover, got: {}", css);
    
    let css = generate_css("bg-contain");
    assert!(css.contains("background-size:contain"), "Expected background-size:contain, got: {}", css);
}

#[test]
fn test_background_blend_modes() {
    // All supported blend modes
    let css = generate_css("bg-blend-normal");
    assert!(css.contains("background-blend-mode:normal"), "Expected background-blend-mode:normal, got: {}", css);
    
    let css = generate_css("bg-blend-multiply");
    assert!(css.contains("background-blend-mode:multiply"), "Expected background-blend-mode:multiply, got: {}", css);
    
    let css = generate_css("bg-blend-screen");
    assert!(css.contains("background-blend-mode:screen"), "Expected background-blend-mode:screen, got: {}", css);
    
    let css = generate_css("bg-blend-overlay");
    assert!(css.contains("background-blend-mode:overlay"), "Expected background-blend-mode:overlay, got: {}", css);
    
    let css = generate_css("bg-blend-darken");
    assert!(css.contains("background-blend-mode:darken"), "Expected background-blend-mode:darken, got: {}", css);
    
    let css = generate_css("bg-blend-lighten");
    assert!(css.contains("background-blend-mode:lighten"), "Expected background-blend-mode:lighten, got: {}", css);
    
    let css = generate_css("bg-blend-color-dodge");
    assert!(css.contains("background-blend-mode:color-dodge"), "Expected background-blend-mode:color-dodge, got: {}", css);
    
    let css = generate_css("bg-blend-color-burn");
    assert!(css.contains("background-blend-mode:color-burn"), "Expected background-blend-mode:color-burn, got: {}", css);
    
    let css = generate_css("bg-blend-hard-light");
    assert!(css.contains("background-blend-mode:hard-light"), "Expected background-blend-mode:hard-light, got: {}", css);
    
    let css = generate_css("bg-blend-soft-light");
    assert!(css.contains("background-blend-mode:soft-light"), "Expected background-blend-mode:soft-light, got: {}", css);
    
    let css = generate_css("bg-blend-difference");
    assert!(css.contains("background-blend-mode:difference"), "Expected background-blend-mode:difference, got: {}", css);
    
    let css = generate_css("bg-blend-exclusion");
    assert!(css.contains("background-blend-mode:exclusion"), "Expected background-blend-mode:exclusion, got: {}", css);
    
    let css = generate_css("bg-blend-hue");
    assert!(css.contains("background-blend-mode:hue"), "Expected background-blend-mode:hue, got: {}", css);
    
    let css = generate_css("bg-blend-saturation");
    assert!(css.contains("background-blend-mode:saturation"), "Expected background-blend-mode:saturation, got: {}", css);
    
    let css = generate_css("bg-blend-color");
    assert!(css.contains("background-blend-mode:color"), "Expected background-blend-mode:color, got: {}", css);
    
    let css = generate_css("bg-blend-luminosity");
    assert!(css.contains("background-blend-mode:luminosity"), "Expected background-blend-mode:luminosity, got: {}", css);
}

#[test]
fn test_background_attachment() {
    let css = generate_css("bg-fixed");
    assert!(css.contains("background-attachment:fixed"), "Expected background-attachment:fixed, got: {}", css);
    
    let css = generate_css("bg-local");
    assert!(css.contains("background-attachment:local"), "Expected background-attachment:local, got: {}", css);
    
    let css = generate_css("bg-scroll");
    assert!(css.contains("background-attachment:scroll"), "Expected background-attachment:scroll, got: {}", css);
}

#[test]
fn test_background_repeat() {
    let css = generate_css("bg-repeat");
    assert!(css.contains("background-repeat:repeat"), "Expected background-repeat:repeat, got: {}", css);
    
    let css = generate_css("bg-no-repeat");
    assert!(css.contains("background-repeat:no-repeat"), "Expected background-repeat:no-repeat, got: {}", css);
    
    let css = generate_css("bg-repeat-x");
    assert!(css.contains("background-repeat:repeat-x"), "Expected background-repeat:repeat-x, got: {}", css);
    
    let css = generate_css("bg-repeat-y");
    assert!(css.contains("background-repeat:repeat-y"), "Expected background-repeat:repeat-y, got: {}", css);
    
    let css = generate_css("bg-repeat-round");
    assert!(css.contains("background-repeat:round"), "Expected background-repeat:round, got: {}", css);
    
    let css = generate_css("bg-repeat-space");
    assert!(css.contains("background-repeat:space"), "Expected background-repeat:space, got: {}", css);
}

#[test]
fn test_background_clip() {
    let css = generate_css("bg-clip-border");
    assert!(css.contains("background-clip:border-box"), "Expected background-clip:border-box, got: {}", css);
    
    let css = generate_css("bg-clip-padding");
    assert!(css.contains("background-clip:padding-box"), "Expected background-clip:padding-box, got: {}", css);
    
    let css = generate_css("bg-clip-content");
    assert!(css.contains("background-clip:content-box"), "Expected background-clip:content-box, got: {}", css);
    
    let css = generate_css("bg-clip-text");
    assert!(css.contains("background-clip:text"), "Expected background-clip:text, got: {}", css);
}

#[test]
fn test_background_origin() {
    let css = generate_css("bg-origin-border");
    assert!(css.contains("background-origin:border-box"), "Expected background-origin:border-box, got: {}", css);
    
    let css = generate_css("bg-origin-padding");
    assert!(css.contains("background-origin:padding-box"), "Expected background-origin:padding-box, got: {}", css);
    
    let css = generate_css("bg-origin-content");
    assert!(css.contains("background-origin:content-box"), "Expected background-origin:content-box, got: {}", css);
}

#[test]
fn test_background_color() {
    // Basic colors
    let css = generate_css("bg-red-500");
    assert!(css.contains("background-color:rgb(239 68 68)"), "Expected background-color:rgb(239 68 68), got: {}", css);
    
    let css = generate_css("bg-blue-500");
    assert!(css.contains("background-color:rgb(59 130 246)"), "Expected background-color:rgb(59 130 246), got: {}", css);
    
    let css = generate_css("bg-transparent");
    assert!(css.contains("background-color:transparent"), "Expected background-color:transparent, got: {}", css);
    
    let css = generate_css("bg-black");
    assert!(css.contains("background-color:rgb(0 0 0)"), "Expected background-color:rgb(0 0 0), got: {}", css);
    
    let css = generate_css("bg-white");
    assert!(css.contains("background-color:rgb(255 255 255)"), "Expected background-color:rgb(255 255 255), got: {}", css);
}

#[test]
fn test_background_gradient() {
    // Gradient directions
    let css = generate_css("bg-gradient-to-t");
    assert!(css.contains("background-image:linear-gradient(to top"), "Expected gradient to top, got: {}", css);
    
    let css = generate_css("bg-gradient-to-b");
    assert!(css.contains("background-image:linear-gradient(to bottom"), "Expected gradient to bottom, got: {}", css);
    
    let css = generate_css("bg-gradient-to-l");
    assert!(css.contains("background-image:linear-gradient(to left"), "Expected gradient to left, got: {}", css);
    
    let css = generate_css("bg-gradient-to-r");
    assert!(css.contains("background-image:linear-gradient(to right"), "Expected gradient to right, got: {}", css);
}

#[test]
fn test_background_position_with_modifiers() {
    // Test with hover modifier
    let css = generate_css("hover:bg-top");
    assert!(css.contains(":hover") && css.contains("background-position:50% 0%"), 
           "Expected hover:background-position:50% 0%, got: {}", css);
    
    let css = generate_css("focus:bg-center");
    assert!(css.contains(":focus") && css.contains("background-position:50% 50%"), 
           "Expected focus:background-position:50% 50%, got: {}", css);
}

#[test]
fn test_background_size_with_modifiers() {
    let css = generate_css("hover:bg-cover");
    assert!(css.contains(":hover") && css.contains("background-size:cover"), 
           "Expected hover:background-size:cover, got: {}", css);
}

#[test]
fn test_background_blend_with_modifiers() {
    let css = generate_css("hover:bg-blend-multiply");
    assert!(css.contains(":hover") && css.contains("background-blend-mode:multiply"), 
           "Expected hover:background-blend-mode:multiply, got: {}", css);
}

#[test]
fn test_mix_blend_mode() {
    // Standard mix-blend-mode utilities (for elements, not backgrounds)
    let css = generate_css("mix-blend-normal");
    assert!(css.contains("mix-blend-mode:normal"), "Expected mix-blend-mode:normal, got: {}", css);
    
    let css = generate_css("mix-blend-multiply");
    assert!(css.contains("mix-blend-mode:multiply"), "Expected mix-blend-mode:multiply, got: {}", css);
    
    let css = generate_css("mix-blend-screen");
    assert!(css.contains("mix-blend-mode:screen"), "Expected mix-blend-mode:screen, got: {}", css);
    
    let css = generate_css("mix-blend-overlay");
    assert!(css.contains("mix-blend-mode:overlay"), "Expected mix-blend-mode:overlay, got: {}", css);
}

#[test]
fn test_background_important_modifier() {
    let css = generate_css("!bg-center");
    assert!(css.contains("background-position:50% 50%") && css.contains("!important"), 
           "Expected background-position:50% 50% !important, got: {}", css);
    
    let css = generate_css("!bg-cover");
    assert!(css.contains("background-size:cover") && css.contains("!important"), 
           "Expected background-size:cover !important, got: {}", css);
    
    let css = generate_css("!bg-blend-multiply");
    assert!(css.contains("background-blend-mode:multiply") && css.contains("!important"), 
           "Expected background-blend-mode:multiply !important, got: {}", css);
}

#[test]
fn test_background_arbitrary_values() {
    // Arbitrary position
    let css = generate_css("bg-[25%_75%]");
    assert!(css.contains("background-position:25% 75%"), "Expected background-position:25% 75%, got: {}", css);
    
    // Arbitrary size - underscores should be converted to spaces in the CSS output
    let css = generate_css("bg-size-[200px_100px]");
    // The process_underscores function should convert _ to space in the CSS value
    assert!(css.contains("background-size:200px 100px"), "Expected background-size:200px 100px, got: {}", css);
    
    let css = generate_css("bg-size-[50%]");
    assert!(css.contains("background-size:50%"), "Expected background-size:50%, got: {}", css);
}

#[test]
fn test_background_image_utilities() {
    let css = generate_css("bg-none");
    assert!(css.contains("background-image:none"), "Expected background-image:none, got: {}", css);
}