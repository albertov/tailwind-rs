use tailwind_rs::*;

/// Test the size utility that sets both width and height simultaneously
#[test]
fn test_size_utility_numeric_values() {
    let mut tw = TailwindBuilder::default();
    
    // Test standard spacing scale values
    assert_eq!(tw.trace("size-0", false).unwrap(), "size-0");
    assert_eq!(tw.trace("size-px", false).unwrap(), "size-px");
    assert_eq!(tw.trace("size-0.5", false).unwrap(), "size-0.5");
    assert_eq!(tw.trace("size-1", false).unwrap(), "size-1");
    assert_eq!(tw.trace("size-2", false).unwrap(), "size-2");
    assert_eq!(tw.trace("size-4", false).unwrap(), "size-4");
    assert_eq!(tw.trace("size-8", false).unwrap(), "size-8");
    assert_eq!(tw.trace("size-12", false).unwrap(), "size-12");
    assert_eq!(tw.trace("size-16", false).unwrap(), "size-16");
    assert_eq!(tw.trace("size-20", false).unwrap(), "size-20");
    assert_eq!(tw.trace("size-24", false).unwrap(), "size-24");
    assert_eq!(tw.trace("size-32", false).unwrap(), "size-32");
    assert_eq!(tw.trace("size-40", false).unwrap(), "size-40");
    assert_eq!(tw.trace("size-48", false).unwrap(), "size-48");
    assert_eq!(tw.trace("size-56", false).unwrap(), "size-56");
    assert_eq!(tw.trace("size-64", false).unwrap(), "size-64");
    assert_eq!(tw.trace("size-72", false).unwrap(), "size-72");
    assert_eq!(tw.trace("size-80", false).unwrap(), "size-80");
    assert_eq!(tw.trace("size-96", false).unwrap(), "size-96");
    
    // Test newly added values
    assert_eq!(tw.trace("size-68", false).unwrap(), "size-68");
    assert_eq!(tw.trace("size-76", false).unwrap(), "size-76");
    assert_eq!(tw.trace("size-84", false).unwrap(), "size-84");
    assert_eq!(tw.trace("size-88", false).unwrap(), "size-88");
    assert_eq!(tw.trace("size-92", false).unwrap(), "size-92");
    
    // Verify CSS output for a few key values
    let css = tw.bundle().unwrap();
    assert!(css.contains(".size-0 { height:0px;width:0px; }") || css.contains(".size-0{height:0px;width:0px}"));
    assert!(css.contains(".size-px { height:1px;width:1px; }") || css.contains(".size-px{height:1px;width:1px}"));
    assert!(css.contains(".size-1 { height:0.25rem;width:0.25rem; }") || css.contains(".size-1{height:0.25rem;width:0.25rem}"));
    assert!(css.contains(".size-96 { height:24rem;width:24rem; }") || css.contains(".size-96{height:24rem;width:24rem}"));
}

#[test]
fn test_size_utility_content_sizing() {
    let mut tw = TailwindBuilder::default();
    
    // Test content-based sizing utilities
    assert_eq!(tw.trace("size-min", false).unwrap(), "size-min");
    assert_eq!(tw.trace("size-max", false).unwrap(), "size-max");
    assert_eq!(tw.trace("size-fit", false).unwrap(), "size-fit");
    assert_eq!(tw.trace("size-auto", false).unwrap(), "size-auto");
    assert_eq!(tw.trace("size-full", false).unwrap(), "size-full");
    
    // Verify CSS output
    let css = tw.bundle().unwrap();
    assert!(css.contains(".size-min { height:min-content;width:min-content; }") || css.contains(".size-min{height:min-content;width:min-content}"));
    assert!(css.contains(".size-max { height:max-content;width:max-content; }") || css.contains(".size-max{height:max-content;width:max-content}"));
    assert!(css.contains(".size-fit { height:fit-content;width:fit-content; }") || css.contains(".size-fit{height:fit-content;width:fit-content}"));
    assert!(css.contains(".size-auto { height:auto;width:auto; }") || css.contains(".size-auto{height:auto;width:auto}"));
    assert!(css.contains(".size-full { height:100%;width:100%; }") || css.contains(".size-full{height:100%;width:100%}"));
}

#[test]
fn test_size_utility_screen() {
    let mut tw = TailwindBuilder::default();
    
    // Test screen-based sizing
    assert_eq!(tw.trace("size-screen", false).unwrap(), "size-screen");
    
    // Verify CSS output - note that width uses vw and height uses vh
    let css = tw.bundle().unwrap();
    assert!(css.contains(".size-screen { height:100vh;width:100vw; }") || css.contains(".size-screen{height:100vh;width:100vw}"));
}

#[test]
fn test_size_utility_arbitrary_values() {
    let mut tw = TailwindBuilder::default();
    
    // Test arbitrary values
    assert_eq!(tw.trace("size-[100px]", false).unwrap(), "size-[100px]");
    assert_eq!(tw.trace("size-[10rem]", false).unwrap(), "size-[10rem]");
    assert_eq!(tw.trace("size-[50%]", false).unwrap(), "size-[50%]");
    assert_eq!(tw.trace("size-[10vh]", false).unwrap(), "size-[10vh]");
    
    // Verify CSS output
    let css = tw.bundle().unwrap();
    assert!(css.contains(r#".size-\[100px\] { height:100px;width:100px; }"#) || css.contains(r#".size-\[100px\]{height:100px;width:100px}"#));
    assert!(css.contains(r#".size-\[10rem\] { height:10rem;width:10rem; }"#) || css.contains(r#".size-\[10rem\]{height:10rem;width:10rem}"#));
    assert!(css.contains(r#".size-\[50\%\] { height:50%;width:50%; }"#) || css.contains(r#".size-\[50\%\]{height:50%;width:50%}"#));
    assert!(css.contains(r#".size-\[10vh\] { height:10vh;width:10vh; }"#) || css.contains(r#".size-\[10vh\]{height:10vh;width:10vh}"#));
}

#[test]
fn test_size_utility_fractions() {
    let mut tw = TailwindBuilder::default();
    
    // Test fraction values
    assert_eq!(tw.trace("size-1/2", false).unwrap(), "size-1/2");
    assert_eq!(tw.trace("size-1/3", false).unwrap(), "size-1/3");
    assert_eq!(tw.trace("size-2/3", false).unwrap(), "size-2/3");
    assert_eq!(tw.trace("size-1/4", false).unwrap(), "size-1/4");
    assert_eq!(tw.trace("size-3/4", false).unwrap(), "size-3/4");
    
    // Verify CSS output
    let css = tw.bundle().unwrap();
    assert!(css.contains(r#".size-1\/2 { height:50%;width:50%; }"#) || css.contains(r#".size-1\/2{height:50%;width:50%}"#));
    // Check for 1/3 fraction (may have slight precision differences)
    assert!(css.contains(r#".size-1\/3 { height:33.33"#) || css.contains(r#".size-1\/3{height:33.33"#));
    // Check for 2/3 fraction (may have slight precision differences)
    assert!(css.contains(r#".size-2\/3 { height:66.66"#) || css.contains(r#".size-2\/3{height:66.66"#));
    assert!(css.contains(r#".size-1\/4 { height:25%;width:25%; }"#) || css.contains(r#".size-1\/4{height:25%;width:25%}"#));
    assert!(css.contains(r#".size-3\/4 { height:75%;width:75%; }"#) || css.contains(r#".size-3\/4{height:75%;width:75%}"#));
}

#[test]
fn test_size_utility_preset_sizes() {
    let mut tw = TailwindBuilder::default();
    
    // Test preset sizes (xs, sm, md, lg, xl, etc.)
    assert_eq!(tw.trace("size-xs", false).unwrap(), "size-xs");
    assert_eq!(tw.trace("size-sm", false).unwrap(), "size-sm");
    assert_eq!(tw.trace("size-md", false).unwrap(), "size-md");
    assert_eq!(tw.trace("size-lg", false).unwrap(), "size-lg");
    assert_eq!(tw.trace("size-xl", false).unwrap(), "size-xl");
    assert_eq!(tw.trace("size-2xl", false).unwrap(), "size-2xl");
    assert_eq!(tw.trace("size-3xl", false).unwrap(), "size-3xl");
    assert_eq!(tw.trace("size-4xl", false).unwrap(), "size-4xl");
    assert_eq!(tw.trace("size-5xl", false).unwrap(), "size-5xl");
    assert_eq!(tw.trace("size-6xl", false).unwrap(), "size-6xl");
    assert_eq!(tw.trace("size-7xl", false).unwrap(), "size-7xl");
    
    // Verify CSS output
    let css = tw.bundle().unwrap();
    assert!(css.contains(".size-xs { height:20rem;width:20rem; }") || css.contains(".size-xs{height:20rem;width:20rem}"));
    assert!(css.contains(".size-sm { height:24rem;width:24rem; }") || css.contains(".size-sm{height:24rem;width:24rem}"));
    assert!(css.contains(".size-md { height:28rem;width:28rem; }") || css.contains(".size-md{height:28rem;width:28rem}"));
    assert!(css.contains(".size-lg { height:32rem;width:32rem; }") || css.contains(".size-lg{height:32rem;width:32rem}"));
    assert!(css.contains(".size-xl { height:36rem;width:36rem; }") || css.contains(".size-xl{height:36rem;width:36rem}"));
    assert!(css.contains(".size-2xl { height:42rem;width:42rem; }") || css.contains(".size-2xl{height:42rem;width:42rem}"));
    assert!(css.contains(".size-3xl { height:48rem;width:48rem; }") || css.contains(".size-3xl{height:48rem;width:48rem}"));
}

#[test]
fn test_width_height_content_sizing() {
    let mut tw = TailwindBuilder::default();
    
    // Test width content sizing
    assert_eq!(tw.trace("w-min", false).unwrap(), "w-min");
    assert_eq!(tw.trace("w-max", false).unwrap(), "w-max");
    assert_eq!(tw.trace("w-fit", false).unwrap(), "w-fit");
    
    // Test height content sizing  
    assert_eq!(tw.trace("h-min", false).unwrap(), "h-min");
    assert_eq!(tw.trace("h-max", false).unwrap(), "h-max");
    assert_eq!(tw.trace("h-fit", false).unwrap(), "h-fit");
    
    // Test min-width content sizing
    assert_eq!(tw.trace("min-w-min", false).unwrap(), "min-w-min");
    assert_eq!(tw.trace("min-w-max", false).unwrap(), "min-w-max");
    assert_eq!(tw.trace("min-w-fit", false).unwrap(), "min-w-fit");
    
    // Test max-width content sizing
    assert_eq!(tw.trace("max-w-min", false).unwrap(), "max-w-min");
    assert_eq!(tw.trace("max-w-max", false).unwrap(), "max-w-max");
    assert_eq!(tw.trace("max-w-fit", false).unwrap(), "max-w-fit");
    
    // Test min-height content sizing
    assert_eq!(tw.trace("min-h-min", false).unwrap(), "min-h-min");
    assert_eq!(tw.trace("min-h-max", false).unwrap(), "min-h-max");
    assert_eq!(tw.trace("min-h-fit", false).unwrap(), "min-h-fit");
    
    // Test max-height content sizing
    assert_eq!(tw.trace("max-h-min", false).unwrap(), "max-h-min");
    assert_eq!(tw.trace("max-h-max", false).unwrap(), "max-h-max");
    assert_eq!(tw.trace("max-h-fit", false).unwrap(), "max-h-fit");
    
    // Verify CSS output
    let css = tw.bundle().unwrap();
    assert!(css.contains(".w-min { width:min-content; }") || css.contains(".w-min{width:min-content}"));
    assert!(css.contains(".w-max { width:max-content; }") || css.contains(".w-max{width:max-content}"));
    assert!(css.contains(".w-fit { width:fit-content; }") || css.contains(".w-fit{width:fit-content}"));
    assert!(css.contains(".h-min { height:min-content; }") || css.contains(".h-min{height:min-content}"));
    assert!(css.contains(".h-max { height:max-content; }") || css.contains(".h-max{height:max-content}"));
    assert!(css.contains(".h-fit { height:fit-content; }") || css.contains(".h-fit{height:fit-content}"));
    assert!(css.contains(".min-w-min { min-width:min-content; }") || css.contains(".min-w-min{min-width:min-content}"));
    assert!(css.contains(".min-w-max { min-width:max-content; }") || css.contains(".min-w-max{min-width:max-content}"));
    assert!(css.contains(".min-w-fit { min-width:fit-content; }") || css.contains(".min-w-fit{min-width:fit-content}"));
    assert!(css.contains(".max-w-min { max-width:min-content; }") || css.contains(".max-w-min{max-width:min-content}"));
    assert!(css.contains(".max-w-max { max-width:max-content; }") || css.contains(".max-w-max{max-width:max-content}"));
    assert!(css.contains(".max-w-fit { max-width:fit-content; }") || css.contains(".max-w-fit{max-width:fit-content}"));
    assert!(css.contains(".min-h-min { min-height:min-content; }") || css.contains(".min-h-min{min-height:min-content}"));
    assert!(css.contains(".min-h-max { min-height:max-content; }") || css.contains(".min-h-max{min-height:max-content}"));
    assert!(css.contains(".min-h-fit { min-height:fit-content; }") || css.contains(".min-h-fit{min-height:fit-content}"));
    assert!(css.contains(".max-h-min { max-height:min-content; }") || css.contains(".max-h-min{max-height:min-content}"));
    assert!(css.contains(".max-h-max { max-height:max-content; }") || css.contains(".max-h-max{max-height:max-content}"));
    assert!(css.contains(".max-h-fit { max-height:fit-content; }") || css.contains(".max-h-fit{max-height:fit-content}"));
}

#[test]
fn test_width_height_extended_scale() {
    let mut tw = TailwindBuilder::default();
    
    // Test newly added values for width
    assert_eq!(tw.trace("w-68", false).unwrap(), "w-68");
    assert_eq!(tw.trace("w-76", false).unwrap(), "w-76");
    assert_eq!(tw.trace("w-84", false).unwrap(), "w-84");
    assert_eq!(tw.trace("w-88", false).unwrap(), "w-88");
    assert_eq!(tw.trace("w-92", false).unwrap(), "w-92");
    
    // Test newly added values for height
    assert_eq!(tw.trace("h-68", false).unwrap(), "h-68");
    assert_eq!(tw.trace("h-76", false).unwrap(), "h-76");
    assert_eq!(tw.trace("h-84", false).unwrap(), "h-84");
    assert_eq!(tw.trace("h-88", false).unwrap(), "h-88");
    assert_eq!(tw.trace("h-92", false).unwrap(), "h-92");
    
    // Verify CSS output
    let css = tw.bundle().unwrap();
    assert!(css.contains(".w-68 { width:17rem; }") || css.contains(".w-68{width:17rem}"));
    assert!(css.contains(".w-76 { width:19rem; }") || css.contains(".w-76{width:19rem}"));
    assert!(css.contains(".w-84 { width:21rem; }") || css.contains(".w-84{width:21rem}"));
    assert!(css.contains(".w-88 { width:22rem; }") || css.contains(".w-88{width:22rem}"));
    assert!(css.contains(".w-92 { width:23rem; }") || css.contains(".w-92{width:23rem}"));
    assert!(css.contains(".h-68 { height:17rem; }") || css.contains(".h-68{height:17rem}"));
    assert!(css.contains(".h-76 { height:19rem; }") || css.contains(".h-76{height:19rem}"));
    assert!(css.contains(".h-84 { height:21rem; }") || css.contains(".h-84{height:21rem}"));
    assert!(css.contains(".h-88 { height:22rem; }") || css.contains(".h-88{height:22rem}"));
    assert!(css.contains(".h-92 { height:23rem; }") || css.contains(".h-92{height:23rem}"));
}

#[test]
fn test_size_utility_with_modifiers() {
    let mut tw = TailwindBuilder::default();
    
    // Test size utilities with responsive modifiers
    assert_eq!(tw.trace("sm:size-10", false).unwrap(), "sm:size-10");
    assert_eq!(tw.trace("md:size-20", false).unwrap(), "md:size-20");
    assert_eq!(tw.trace("lg:size-32", false).unwrap(), "lg:size-32");
    
    // Test size utilities with hover modifier
    assert_eq!(tw.trace("hover:size-full", false).unwrap(), "hover:size-full");
    
    // Test size utilities with focus modifier
    assert_eq!(tw.trace("focus:size-auto", false).unwrap(), "focus:size-auto");
    
    // Verify CSS output includes proper selectors
    let css = tw.bundle().unwrap();
    // Media query may have different formatting
    assert!(css.contains("@media (min-width: 640px)"));
    assert!(css.contains(".sm\\:size-10 { height:2.5rem;width:2.5rem; }") || css.contains(".sm\\:size-10{height:2.5rem;width:2.5rem}"));
    assert!(css.contains(".hover\\:size-full:hover { height:100%;width:100%; }") || css.contains(".hover\\:size-full:hover{height:100%;width:100%}"));
    assert!(css.contains(".focus\\:size-auto:focus { height:auto;width:auto; }") || css.contains(".focus\\:size-auto:focus{height:auto;width:auto}"));
}