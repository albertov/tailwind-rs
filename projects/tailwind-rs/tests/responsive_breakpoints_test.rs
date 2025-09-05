use tailwind_rs::TailwindBuilder;

// ============================================================================
// COMPREHENSIVE RESPONSIVE BREAKPOINT TESTS
// ============================================================================
// These tests ensure that all Tailwind responsive breakpoints work correctly
// with their appropriate media queries and CSS generation.
//
// Default Tailwind breakpoints:
// - sm: 640px
// - md: 768px  
// - lg: 1024px
// - xl: 1280px
// - 2xl: 1536px

// ============================================================================
// INDIVIDUAL BREAKPOINT TESTS
// ============================================================================

#[test]
fn test_sm_breakpoint_utilities() {
    let mut tw = TailwindBuilder::default();
    
    // Test various utilities with sm breakpoint
    tw.trace("sm:flex", false).unwrap();
    tw.trace("sm:hidden", false).unwrap();
    tw.trace("sm:block", false).unwrap();
    tw.trace("sm:p-4", false).unwrap();
    tw.trace("sm:text-lg", false).unwrap();
    tw.trace("sm:bg-blue-500", false).unwrap();
    tw.trace("sm:w-full", false).unwrap();
    tw.trace("sm:mx-auto", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify media query for sm breakpoint
    assert!(css.contains("@media (min-width: 640px)"), "Missing sm breakpoint media query");
    
    // Verify all utilities are generated with correct breakpoint
    assert!(css.contains(".sm\\:flex"), "Missing .sm:flex");
    assert!(css.contains(".sm\\:hidden"), "Missing .sm:hidden");
    assert!(css.contains(".sm\\:block"), "Missing .sm:block");
    assert!(css.contains(".sm\\:p-4"), "Missing .sm:p-4");
    assert!(css.contains(".sm\\:text-lg"), "Missing .sm:text-lg");
    assert!(css.contains(".sm\\:bg-blue-500"), "Missing .sm:bg-blue-500");
    assert!(css.contains(".sm\\:w-full"), "Missing .sm:w-full");
    assert!(css.contains(".sm\\:mx-auto"), "Missing .sm:mx-auto");
    
    // Verify CSS properties are correct
    assert!(css.contains("display:flex"), "Missing display:flex for sm:flex");
    assert!(css.contains("display:none"), "Missing display:none for sm:hidden");
    assert!(css.contains("display:block"), "Missing display:block for sm:block");
    assert!(css.contains("padding:1rem"), "Missing padding for sm:p-4");
    assert!(css.contains("font-size:1.125rem"), "Missing font-size for sm:text-lg");
    assert!(css.contains("width:100%"), "Missing width:100% for sm:w-full");
}

#[test]
fn test_md_breakpoint_utilities() {
    let mut tw = TailwindBuilder::default();
    
    // Test various utilities with md breakpoint
    tw.trace("md:grid", false).unwrap();
    tw.trace("md:flex", false).unwrap();
    tw.trace("md:hidden", false).unwrap();
    tw.trace("md:grid-cols-2", false).unwrap();
    tw.trace("md:gap-4", false).unwrap();
    tw.trace("md:text-xl", false).unwrap();
    tw.trace("md:py-8", false).unwrap();
    tw.trace("md:container", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify media query for md breakpoint
    assert!(css.contains("@media (min-width: 768px)"), "Missing md breakpoint media query");
    
    // Verify all utilities are generated
    assert!(css.contains(".md\\:grid"), "Missing .md:grid");
    assert!(css.contains(".md\\:flex"), "Missing .md:flex");
    assert!(css.contains(".md\\:hidden"), "Missing .md:hidden");
    assert!(css.contains(".md\\:grid-cols-2"), "Missing .md:grid-cols-2");
    assert!(css.contains(".md\\:gap-\\[1rem\\]"), "Missing .md:gap-4 (renders as gap-[1rem])");
    assert!(css.contains(".md\\:text-xl"), "Missing .md:text-xl");
    assert!(css.contains(".md\\:py-8"), "Missing .md:py-8");
    assert!(css.contains(".md\\:container"), "Missing .md:container");
    
    // Verify CSS properties
    assert!(css.contains("display:grid"), "Missing display:grid for md:grid");
    assert!(css.contains("grid-template-columns:repeat(2,minmax(0,1fr))"), "Missing grid columns");
    assert!(css.contains("gap:1rem"), "Missing gap for md:gap-4");
    assert!(css.contains("font-size:1.25rem"), "Missing font-size for md:text-xl");
}

#[test]
fn test_lg_breakpoint_utilities() {
    let mut tw = TailwindBuilder::default();
    
    // Test various utilities with lg breakpoint
    tw.trace("lg:flex", false).unwrap();
    tw.trace("lg:grid-cols-3", false).unwrap();
    tw.trace("lg:w-1/2", false).unwrap();
    tw.trace("lg:text-2xl", false).unwrap();
    tw.trace("lg:px-12", false).unwrap();
    tw.trace("lg:max-w-4xl", false).unwrap();
    tw.trace("lg:justify-center", false).unwrap();
    tw.trace("lg:items-center", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify media query for lg breakpoint
    assert!(css.contains("@media (min-width: 1024px)"), "Missing lg breakpoint media query");
    
    // Verify all utilities are generated
    assert!(css.contains(".lg\\:flex"), "Missing .lg:flex");
    assert!(css.contains(".lg\\:grid-cols-3"), "Missing .lg:grid-cols-3");
    assert!(css.contains(".lg\\:w-1\\/2"), "Missing .lg:w-1/2");
    assert!(css.contains(".lg\\:text-2xl"), "Missing .lg:text-2xl");
    assert!(css.contains(".lg\\:px-12"), "Missing .lg:px-12");
    assert!(css.contains(".lg\\:max-w-4xl"), "Missing .lg:max-w-4xl");
    assert!(css.contains(".lg\\:justify-center"), "Missing .lg:justify-center");
    assert!(css.contains(".lg\\:items-center"), "Missing .lg:items-center");
    
    // Verify CSS properties
    assert!(css.contains("grid-template-columns:repeat(3,minmax(0,1fr))"), "Missing grid-cols-3");
    assert!(css.contains("width:50%"), "Missing width:50% for lg:w-1/2");
    assert!(css.contains("font-size:1.5rem"), "Missing font-size for lg:text-2xl");
    assert!(css.contains("padding-left:3rem") && css.contains("padding-right:3rem"), 
            "Missing padding for lg:px-12");
    assert!(css.contains("justify-content:center"), "Missing justify-content:center");
    assert!(css.contains("align-items:center"), "Missing align-items:center");
}

#[test]
fn test_xl_breakpoint_utilities() {
    let mut tw = TailwindBuilder::default();
    
    // Test various utilities with xl breakpoint
    tw.trace("xl:container", false).unwrap();
    tw.trace("xl:text-3xl", false).unwrap();
    tw.trace("xl:grid-cols-4", false).unwrap();
    tw.trace("xl:gap-8", false).unwrap();
    tw.trace("xl:max-w-7xl", false).unwrap();
    tw.trace("xl:px-16", false).unwrap();
    tw.trace("xl:flex-row", false).unwrap();
    tw.trace("xl:space-x-8", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify media query for xl breakpoint
    assert!(css.contains("@media (min-width: 1280px)"), "Missing xl breakpoint media query");
    
    // Verify all utilities are generated
    assert!(css.contains(".xl\\:container"), "Missing .xl:container");
    assert!(css.contains(".xl\\:text-3xl"), "Missing .xl:text-3xl");
    assert!(css.contains(".xl\\:grid-cols-4"), "Missing .xl:grid-cols-4");
    assert!(css.contains(".xl\\:gap-\\[2rem\\]"), "Missing .xl:gap-8 (renders as gap-[2rem])");
    assert!(css.contains(".xl\\:max-w-7xl"), "Missing .xl:max-w-7xl");
    assert!(css.contains(".xl\\:px-16"), "Missing .xl:px-16");
    assert!(css.contains(".xl\\:flex-row"), "Missing .xl:flex-row");
    assert!(css.contains(".xl\\:space-x-8"), "Missing .xl:space-x-8");
    
    // Verify CSS properties
    assert!(css.contains("grid-template-columns:repeat(4,minmax(0,1fr))"), "Missing grid-cols-4");
    assert!(css.contains("gap:2rem"), "Missing gap for xl:gap-8");
    assert!(css.contains("font-size:1.875rem"), "Missing font-size for xl:text-3xl");
    assert!(css.contains("padding-left:4rem") && css.contains("padding-right:4rem"), 
            "Missing padding for xl:px-16");
    assert!(css.contains("flex-direction:row"), "Missing flex-direction:row");
}

#[test]
fn test_2xl_breakpoint_utilities() {
    let mut tw = TailwindBuilder::default();
    
    // Test various utilities with 2xl breakpoint
    tw.trace("2xl:max-w-7xl", false).unwrap();
    tw.trace("2xl:px-8", false).unwrap();
    tw.trace("2xl:text-4xl", false).unwrap();
    tw.trace("2xl:grid-cols-6", false).unwrap();
    tw.trace("2xl:gap-12", false).unwrap();
    tw.trace("2xl:container", false).unwrap();
    tw.trace("2xl:mx-auto", false).unwrap();
    tw.trace("2xl:py-16", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify media query for 2xl breakpoint
    assert!(css.contains("@media (min-width: 1536px)"), "Missing 2xl breakpoint media query");
    
    // Verify all utilities are generated (2xl uses .2xl\: not .\32xl\:)
    assert!(css.contains(".2xl\\:max-w-7xl"), "Missing .2xl:max-w-7xl");
    assert!(css.contains(".2xl\\:px-8"), "Missing .2xl:px-8");
    assert!(css.contains(".2xl\\:text-4xl"), "Missing .2xl:text-4xl");
    assert!(css.contains(".2xl\\:grid-cols-6"), "Missing .2xl:grid-cols-6");
    assert!(css.contains(".2xl\\:gap-\\[3rem\\]"), "Missing .2xl:gap-12 (renders as gap-[3rem])");
    assert!(css.contains(".2xl\\:container"), "Missing .2xl:container");
    assert!(css.contains(".2xl\\:mx-auto"), "Missing .2xl:mx-auto");
    assert!(css.contains(".2xl\\:py-16"), "Missing .2xl:py-16");
    
    // Verify CSS properties
    assert!(css.contains("grid-template-columns:repeat(6,minmax(0,1fr))"), "Missing grid-cols-6");
    assert!(css.contains("gap:3rem"), "Missing gap for 2xl:gap-12");
    assert!(css.contains("font-size:2.25rem"), "Missing font-size for 2xl:text-4xl");
    assert!(css.contains("padding-left:2rem") && css.contains("padding-right:2rem"), 
            "Missing padding for 2xl:px-8");
    assert!(css.contains("padding-top:4rem") && css.contains("padding-bottom:4rem"), 
            "Missing padding for 2xl:py-16");
}

// ============================================================================
// BREAKPOINT ORDERING TESTS (Mobile-first approach)
// ============================================================================

#[test]
fn test_mobile_first_breakpoint_ordering() {
    let mut tw = TailwindBuilder::default();
    
    // Test mobile-first cascade with same property at different breakpoints
    tw.trace("text-sm sm:text-base md:text-lg lg:text-xl xl:text-2xl 2xl:text-3xl", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Base (no media query)
    assert!(css.contains(".text-sm") && css.contains("font-size:0.875rem"), 
            "Missing base text-sm");
    
    // Verify each breakpoint overrides with correct media query
    assert!(css.contains("@media (min-width: 640px)") && css.contains(".sm\\:text-base"), 
            "Missing sm:text-base");
    assert!(css.contains("@media (min-width: 768px)") && css.contains(".md\\:text-lg"), 
            "Missing md:text-lg");
    assert!(css.contains("@media (min-width: 1024px)") && css.contains(".lg\\:text-xl"), 
            "Missing lg:text-xl");
    assert!(css.contains("@media (min-width: 1280px)") && css.contains(".xl\\:text-2xl"), 
            "Missing xl:text-2xl");
    assert!(css.contains("@media (min-width: 1536px)") && css.contains(".2xl\\:text-3xl"), 
            "Missing 2xl:text-3xl");
}

#[test]
fn test_display_property_responsive_progression() {
    let mut tw = TailwindBuilder::default();
    
    // Test common responsive pattern: hidden on mobile, visible on larger screens
    tw.trace("hidden sm:block md:flex lg:grid xl:inline-flex 2xl:flex", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify base state
    assert!(css.contains(".hidden") && css.contains("display:none"), 
            "Missing base hidden");
    
    // Verify progressive display changes
    assert!(css.contains("@media (min-width: 640px)") && css.contains(".sm\\:block") && 
            css.contains("display:block"), "Missing sm:block");
    assert!(css.contains("@media (min-width: 768px)") && css.contains(".md\\:flex") && 
            css.contains("display:flex"), "Missing md:flex");
    assert!(css.contains("@media (min-width: 1024px)") && css.contains(".lg\\:grid") && 
            css.contains("display:grid"), "Missing lg:grid");
    assert!(css.contains("@media (min-width: 1280px)") && css.contains(".xl\\:inline-flex") && 
            css.contains("display:inline-flex"), "Missing xl:inline-flex");
    assert!(css.contains("@media (min-width: 1536px)") && css.contains(".2xl\\:flex") && 
            css.contains("display:flex"), "Missing 2xl:flex");
}

#[test]
fn test_mixed_responsive_utilities() {
    let mut tw = TailwindBuilder::default();
    
    // Test different utilities at different breakpoints
    tw.trace("p-2 sm:p-4 md:px-6 lg:py-8 xl:p-10 2xl:p-12", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Base padding
    assert!(css.contains(".p-2") && css.contains("padding:0.5rem"), 
            "Missing base p-2");
    
    // Responsive padding variations
    assert!(css.contains("@media (min-width: 640px)") && css.contains(".sm\\:p-4") && 
            css.contains("padding:1rem"), "Missing sm:p-4");
    assert!(css.contains("@media (min-width: 768px)") && css.contains(".md\\:px-6") && 
            css.contains("padding-left:1.5rem") && css.contains("padding-right:1.5rem"), 
            "Missing md:px-6");
    assert!(css.contains("@media (min-width: 1024px)") && css.contains(".lg\\:py-8") && 
            css.contains("padding-top:2rem") && css.contains("padding-bottom:2rem"), 
            "Missing lg:py-8");
}

// ============================================================================
// BREAKPOINT + VARIANT COMBINATION TESTS
// ============================================================================

#[test]
fn test_breakpoint_with_hover_variant() {
    let mut tw = TailwindBuilder::default();
    
    // Test hover variant at different breakpoints
    tw.trace("sm:hover:bg-blue-500", false).unwrap();
    tw.trace("md:hover:font-bold", false).unwrap();
    tw.trace("lg:hover:scale-105", false).unwrap();
    tw.trace("xl:hover:shadow-lg", false).unwrap();
    tw.trace("2xl:hover:border-2", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify each breakpoint + hover combination
    // Note: hover variants also add @media (hover: hover) for accessibility
    assert!(css.contains("@media (min-width: 640px)") && 
            css.contains("@media (hover: hover)") &&
            css.contains(".sm\\:hover\\:bg-blue-500:hover"), 
            "Missing sm:hover:bg-blue-500");
    assert!(css.contains("@media (min-width: 768px)") && 
            css.contains("@media (hover: hover)") &&
            css.contains(".md\\:hover\\:font-\\[700\\]:hover"), 
            "Missing md:hover:font-bold (renders as font-[700])");
    assert!(css.contains("@media (min-width: 1024px)") && 
            css.contains("@media (hover: hover)") &&
            css.contains(".lg\\:hover\\:scale-105:hover"), 
            "Missing lg:hover:scale-105");
    assert!(css.contains("@media (min-width: 1280px)") && 
            css.contains("@media (hover: hover)") &&
            css.contains(".xl\\:hover\\:shadow-lg:hover"), 
            "Missing xl:hover:shadow-lg");
    assert!(css.contains("@media (min-width: 1536px)") && 
            css.contains("@media (hover: hover)") &&
            css.contains(".2xl\\:hover\\:border-\\[2px\\]:hover"), 
            "Missing 2xl:hover:border-2 (renders as border-[2px])");
}

#[test]
fn test_breakpoint_with_dark_variant() {
    let mut tw = TailwindBuilder::default();
    
    // Test dark mode at different breakpoints
    tw.trace("sm:dark:bg-gray-800", false).unwrap();
    tw.trace("md:dark:text-gray-200", false).unwrap();
    tw.trace("lg:dark:border-gray-600", false).unwrap();
    tw.trace("xl:dark:shadow-xl", false).unwrap();
    tw.trace("2xl:dark:rounded-xl", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify media query nesting for responsive + dark mode
    assert!(css.contains("@media (min-width: 640px)") && 
            css.contains("@media (prefers-color-scheme: dark)") &&
            css.contains(".sm\\:dark\\:bg-gray-800"), 
            "Missing sm:dark:bg-gray-800");
    assert!(css.contains("@media (min-width: 768px)") && 
            css.contains(".md\\:dark\\:text-gray-200"), 
            "Missing md:dark:text-gray-200");
    assert!(css.contains("@media (min-width: 1024px)") && 
            css.contains(".lg\\:dark\\:border-gray-600"), 
            "Missing lg:dark:border-gray-600");
}

#[test]
fn test_breakpoint_with_focus_variant() {
    let mut tw = TailwindBuilder::default();
    
    // Test focus state at different breakpoints
    tw.trace("sm:focus:ring-2", false).unwrap();
    tw.trace("md:focus:ring-4", false).unwrap();
    tw.trace("lg:focus:outline-none", false).unwrap();
    tw.trace("xl:focus:border-blue-500", false).unwrap();
    tw.trace("2xl:focus:shadow-outline", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify breakpoint + focus combinations
    assert!(css.contains("@media (min-width: 640px)") && 
            css.contains(".sm\\:focus\\:ring-2:focus"), 
            "Missing sm:focus:ring-2");
    assert!(css.contains("@media (min-width: 768px)") && 
            css.contains(".md\\:focus\\:ring-4:focus"), 
            "Missing md:focus:ring-4");
    assert!(css.contains("@media (min-width: 1024px)") && 
            css.contains(".lg\\:focus\\:outline-none:focus"), 
            "Missing lg:focus:outline-none");
}

#[test]
fn test_breakpoint_with_group_variant() {
    let mut tw = TailwindBuilder::default();
    
    // Test group hover at different breakpoints
    tw.trace("sm:group-hover:bg-blue-100", false).unwrap();
    tw.trace("md:group-hover:font-semibold", false).unwrap();
    tw.trace("lg:group-hover:scale-110", false).unwrap();
    tw.trace("xl:group-hover:shadow-2xl", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify breakpoint + group-hover combinations
    // Note: The actual CSS uses descendant selector pattern
    // Also includes @media (hover: hover) for accessibility
    assert!(css.contains("@media (min-width: 640px)") && 
            css.contains("@media (hover: hover)") &&
            css.contains(".group:hover .sm\\:group-hover\\:bg-blue-100"), 
            "Missing sm:group-hover:bg-blue-100");
    assert!(css.contains("@media (min-width: 768px)") && 
            css.contains("@media (hover: hover)") &&
            css.contains(".group:hover .md\\:group-hover\\:font-\\[600\\]"), 
            "Missing md:group-hover:font-semibold (renders as font-[600])");
}

#[test]
fn test_multiple_variants_with_breakpoints() {
    let mut tw = TailwindBuilder::default();
    
    // Test complex combinations: breakpoint + dark + hover
    tw.trace("sm:dark:hover:bg-gray-700", false).unwrap();
    tw.trace("md:hover:dark:text-gray-100", false).unwrap();
    tw.trace("lg:focus:dark:ring-white", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify complex variant stacking
    assert!(css.contains("@media (min-width: 640px)"), "Missing sm media query");
    assert!(css.contains("@media (prefers-color-scheme: dark)"), "Missing dark mode query");
    assert!(css.contains(".sm\\:dark\\:hover\\:bg-gray-700:hover"), 
            "Missing sm:dark:hover:bg-gray-700");
}

// ============================================================================
// ARBITRARY BREAKPOINT TESTS
// ============================================================================

#[test]
fn test_arbitrary_min_width_breakpoint() {
    let mut tw = TailwindBuilder::default();
    
    // Test arbitrary min-width breakpoints
    tw.trace("min-[400px]:flex", false).unwrap();
    tw.trace("min-[600px]:grid", false).unwrap();
    tw.trace("min-[900px]:hidden", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify arbitrary min-width media queries
    assert!(css.contains("@media (min-width: 400px)"), "Missing min-[400px] query");
    assert!(css.contains("@media (min-width: 600px)"), "Missing min-[600px] query");
    assert!(css.contains("@media (min-width: 900px)"), "Missing min-[900px] query");
}

#[test]
fn test_arbitrary_max_width_breakpoint() {
    let mut tw = TailwindBuilder::default();
    
    // Test arbitrary max-width breakpoints
    tw.trace("max-[400px]:block", false).unwrap();
    tw.trace("max-[600px]:hidden", false).unwrap();
    tw.trace("max-[800px]:text-sm", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify arbitrary max-width media queries
    assert!(css.contains("@media (max-width: 400px)"), "Missing max-[400px] query");
    assert!(css.contains("@media (max-width: 600px)"), "Missing max-[600px] query");
    assert!(css.contains("@media (max-width: 800px)"), "Missing max-[800px] query");
}

// ============================================================================
// CONTAINER QUERY VS MEDIA QUERY TESTS
// ============================================================================

#[test]
fn test_container_query_differs_from_media_query() {
    let mut tw = TailwindBuilder::default();
    
    // Media query breakpoint
    tw.trace("lg:flex", false).unwrap();
    // Container query
    tw.trace("@lg:flex", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Media query should use min-width
    assert!(css.contains("@media (min-width: 1024px)"), "Missing media query for lg:");
    assert!(css.contains(".lg\\:flex"), "Missing .lg:flex");
    
    // Container query should use @container with different value (48rem for lg in container queries)
    assert!(css.contains("@container (min-width: 48rem)"), 
            "Container query should use @container with 48rem. Got CSS: {}", css);
    assert!(css.contains(".\\@lg\\:flex"), "Missing .@lg:flex class. Got CSS: {}", css);
    
    // Verify they are indeed different
    assert!(css.contains("@media") && css.contains("@container"), 
            "CSS should contain both media and container queries");
}

// ============================================================================
// VISUAL GRID EVOLUTION TEST
// ============================================================================

#[test]
fn test_responsive_grid_evolution() {
    let mut tw = TailwindBuilder::default();
    
    // Test grid column evolution across breakpoints
    tw.trace("grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Base grid setup
    assert!(css.contains(".grid") && css.contains("display:grid"), 
            "Missing base grid");
    assert!(css.contains(".grid-cols-1") && 
            css.contains("grid-template-columns:repeat(1,minmax(0,1fr))"), 
            "Missing grid-cols-1");
    
    // Verify progressive grid columns
    assert!(css.contains("@media (min-width: 640px)") && 
            css.contains(".sm\\:grid-cols-2") &&
            css.contains("grid-template-columns:repeat(2,minmax(0,1fr))"), 
            "Missing sm:grid-cols-2");
    
    assert!(css.contains("@media (min-width: 768px)") && 
            css.contains(".md\\:grid-cols-3") &&
            css.contains("grid-template-columns:repeat(3,minmax(0,1fr))"), 
            "Missing md:grid-cols-3");
    
    assert!(css.contains("@media (min-width: 1024px)") && 
            css.contains(".lg\\:grid-cols-4") &&
            css.contains("grid-template-columns:repeat(4,minmax(0,1fr))"), 
            "Missing lg:grid-cols-4");
    
    assert!(css.contains("@media (min-width: 1280px)") && 
            css.contains(".xl\\:grid-cols-5") &&
            css.contains("grid-template-columns:repeat(5,minmax(0,1fr))"), 
            "Missing xl:grid-cols-5");
    
    assert!(css.contains("@media (min-width: 1536px)") && 
            css.contains(".2xl\\:grid-cols-6") &&
            css.contains("grid-template-columns:repeat(6,minmax(0,1fr))"), 
            "Missing 2xl:grid-cols-6");
}

#[test]
fn test_responsive_spacing_evolution() {
    let mut tw = TailwindBuilder::default();
    
    // Test spacing that increases with screen size
    tw.trace("gap-2 sm:gap-3 md:gap-4 lg:gap-6 xl:gap-8 2xl:gap-10", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify progressive gap increases
    assert!(css.contains(".gap-\\[0\\.5rem\\]") && css.contains("gap:0.5rem"), 
            "Missing base gap-2");
    assert!(css.contains("@media (min-width: 640px)") && 
            css.contains(".sm\\:gap-\\[0\\.75rem\\]") && css.contains("gap:0.75rem"), 
            "Missing sm:gap-3");
    assert!(css.contains("@media (min-width: 768px)") && 
            css.contains(".md\\:gap-\\[1rem\\]") && css.contains("gap:1rem"), 
            "Missing md:gap-4");
    assert!(css.contains("@media (min-width: 1024px)") && 
            css.contains(".lg\\:gap-\\[1\\.5rem\\]") && css.contains("gap:1.5rem"), 
            "Missing lg:gap-6");
    assert!(css.contains("@media (min-width: 1280px)") && 
            css.contains(".xl\\:gap-\\[2rem\\]") && css.contains("gap:2rem"), 
            "Missing xl:gap-8");
    assert!(css.contains("@media (min-width: 1536px)") && 
            css.contains(".2xl\\:gap-\\[2\\.5rem\\]") && css.contains("gap:2.5rem"), 
            "Missing 2xl:gap-10");
}

// ============================================================================
// COMPREHENSIVE COVERAGE TEST
// ============================================================================

#[test]
fn test_all_breakpoints_comprehensive() {
    let mut tw = TailwindBuilder::default();
    
    // Test a wide variety of utilities across all breakpoints
    let test_classes = vec![
        // Layout
        "sm:container", "md:container", "lg:container", "xl:container", "2xl:container",
        "sm:block", "md:inline-block", "lg:inline", "xl:flex", "2xl:inline-flex",
        "sm:grid", "md:hidden", "lg:table", "xl:table-cell", "2xl:flow-root",
        
        // Flexbox
        "sm:flex-row", "md:flex-col", "lg:flex-wrap", "xl:flex-nowrap", "2xl:flex-row-reverse",
        "sm:justify-start", "md:justify-center", "lg:justify-end", "xl:justify-between", "2xl:justify-around",
        "sm:items-start", "md:items-center", "lg:items-end", "xl:items-baseline", "2xl:items-stretch",
        
        // Grid
        "sm:grid-cols-1", "md:grid-cols-2", "lg:grid-cols-3", "xl:grid-cols-4", "2xl:grid-cols-5",
        "sm:col-span-1", "md:col-span-2", "lg:col-span-3", "xl:col-span-4", "2xl:col-span-5",
        
        // Spacing
        "sm:p-1", "md:p-2", "lg:p-4", "xl:p-6", "2xl:p-8",
        "sm:m-1", "md:m-2", "lg:m-4", "xl:m-6", "2xl:m-8",
        "sm:gap-1", "md:gap-2", "lg:gap-4", "xl:gap-6", "2xl:gap-8",
        
        // Sizing
        "sm:w-full", "md:w-1/2", "lg:w-1/3", "xl:w-1/4", "2xl:w-1/5",
        "sm:h-10", "md:h-20", "lg:h-32", "xl:h-40", "2xl:h-48",
        "sm:max-w-sm", "md:max-w-md", "lg:max-w-lg", "xl:max-w-xl", "2xl:max-w-2xl",
        
        // Typography
        "sm:text-xs", "md:text-sm", "lg:text-base", "xl:text-lg", "2xl:text-xl",
        "sm:font-normal", "md:font-medium", "lg:font-semibold", "xl:font-bold", "2xl:font-extrabold",
        "sm:leading-tight", "md:leading-snug", "lg:leading-normal", "xl:leading-relaxed", "2xl:leading-loose",
        
        // Colors
        "sm:text-gray-500", "md:text-blue-500", "lg:text-green-500", "xl:text-red-500", "2xl:text-purple-500",
        "sm:bg-gray-100", "md:bg-blue-100", "lg:bg-green-100", "xl:bg-red-100", "2xl:bg-purple-100",
        
        // Borders
        "sm:border", "md:border-2", "lg:border-4", "xl:border-8", "2xl:border-0",
        "sm:rounded", "md:rounded-md", "lg:rounded-lg", "xl:rounded-xl", "2xl:rounded-2xl",
        
        // Effects
        "sm:shadow-sm", "md:shadow", "lg:shadow-md", "xl:shadow-lg", "2xl:shadow-xl",
        "sm:opacity-25", "md:opacity-50", "lg:opacity-75", "xl:opacity-90", "2xl:opacity-100",
    ];
    
    // Trace all classes
    for class in &test_classes {
        tw.trace(class, false).unwrap();
    }
    
    let css = tw.bundle().unwrap();
    
    // Verify all breakpoint media queries are present
    assert!(css.contains("@media (min-width: 640px)"), "Missing sm breakpoint");
    assert!(css.contains("@media (min-width: 768px)"), "Missing md breakpoint");
    assert!(css.contains("@media (min-width: 1024px)"), "Missing lg breakpoint");
    assert!(css.contains("@media (min-width: 1280px)"), "Missing xl breakpoint");
    assert!(css.contains("@media (min-width: 1536px)"), "Missing 2xl breakpoint");
    
    // Spot check some classes from each category
    assert!(css.contains(".sm\\:container"), "Missing sm:container");
    assert!(css.contains(".md\\:flex-col"), "Missing md:flex-col");
    assert!(css.contains(".lg\\:grid-cols-3"), "Missing lg:grid-cols-3");
    assert!(css.contains(".xl\\:p-6"), "Missing xl:p-6");
    assert!(css.contains(".2xl\\:text-xl"), "Missing 2xl:text-xl");
    assert!(css.contains(".sm\\:shadow-sm"), "Missing sm:shadow-sm");
}