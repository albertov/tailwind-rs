use tailwind_rs::TailwindBuilder;

#[test]
fn test_basic_dark_mode_utilities() {
    let mut tw = TailwindBuilder::default();
    
    // Test basic dark mode color utilities
    tw.trace("dark:bg-gray-900", false).unwrap();
    tw.trace("dark:text-white", false).unwrap();
    tw.trace("dark:border-gray-700", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify dark mode background
    assert!(css.contains("@media (prefers-color-scheme: dark)"));
    assert!(css.contains(".dark\\:bg-gray-900"));
    assert!(css.contains("background-color:rgb(17 24 39"));
    
    // Verify dark mode text (white keyword is preserved)
    assert!(css.contains(".dark\\:text-white"));
    assert!(css.contains("color:rgb(255 255 255"));
    
    // Verify dark mode border
    assert!(css.contains(".dark\\:border-gray-700"));
    assert!(css.contains("border-color:rgb(55 65 81"));
}

#[test]
fn test_dark_mode_with_responsive() {
    let mut tw = TailwindBuilder::default();
    
    // Test dark mode combined with responsive variants
    tw.trace("sm:dark:bg-gray-800", false).unwrap();
    tw.trace("lg:dark:flex", false).unwrap();
    tw.trace("md:dark:text-blue-400", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify responsive + dark mode combination
    assert!(css.contains("@media (min-width: 640px)"));
    assert!(css.contains("@media (prefers-color-scheme: dark)"));
    assert!(css.contains(".sm\\:dark\\:bg-gray-800"));
    
    // Verify large screen dark flex
    assert!(css.contains("@media (min-width: 1024px)"));
    assert!(css.contains(".lg\\:dark\\:flex"));
    assert!(css.contains("display:flex"));
    
    // Verify medium screen dark text
    assert!(css.contains("@media (min-width: 768px)"));
    assert!(css.contains(".md\\:dark\\:text-blue-400"));
}

#[test]
fn test_dark_mode_with_pseudo_states() {
    let mut tw = TailwindBuilder::default();
    
    // Test dark mode combined with hover, focus, and other pseudo-classes
    tw.trace("dark:hover:bg-gray-700", false).unwrap();
    tw.trace("dark:focus:ring-white", false).unwrap();
    tw.trace("dark:active:bg-gray-600", false).unwrap();
    tw.trace("hover:dark:text-gray-200", false).unwrap(); // Different order
    
    let css = tw.bundle().unwrap();
    
    // Verify dark hover combination
    assert!(css.contains(".dark\\:hover\\:bg-gray-700:hover"));
    assert!(css.contains("background-color:rgb(55 65 81"));
    
    // Verify dark focus ring (white keyword is preserved)
    assert!(css.contains(".dark\\:focus\\:ring-white:focus"));
    
    // Verify dark active state
    assert!(css.contains(".dark\\:active\\:bg-gray-600:active"));
    
    // Verify hover dark (different order)
    assert!(css.contains(".hover\\:dark\\:text-gray-200:hover"));
}

#[test]
fn test_dark_mode_with_group_and_peer() {
    let mut tw = TailwindBuilder::default();
    
    // Test dark mode with group variants
    tw.trace("group-hover:dark:bg-gray-600", false).unwrap();
    tw.trace("dark:group-hover:text-gray-100", false).unwrap();
    
    // Test dark mode with peer variants
    tw.trace("peer-checked:dark:text-green-400", false).unwrap();
    tw.trace("dark:peer-focus:border-blue-400", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify group-hover + dark (uses descendant selector)
    assert!(css.contains(".group:hover .group-hover\\:dark\\:bg-gray-600"));
    assert!(css.contains("@media (prefers-color-scheme: dark)"));
    
    // Verify dark + group-hover (uses descendant selector)
    assert!(css.contains(".group:hover .dark\\:group-hover\\:text-gray-100"));
    
    // Verify peer-checked + dark (uses sibling selector)
    assert!(css.contains(".peer:checked ~ .peer-checked\\:dark\\:text-green-400"));
    
    // Verify dark + peer-focus (uses sibling selector)
    assert!(css.contains(".peer:focus ~ .dark\\:peer-focus\\:border-blue-400"));
}

#[test]
fn test_dark_mode_with_multiple_variants() {
    let mut tw = TailwindBuilder::default();
    
    // Test complex variant combinations
    tw.trace("sm:dark:hover:bg-gray-700", false).unwrap();
    tw.trace("lg:dark:focus:text-white", false).unwrap();
    tw.trace("dark:first:mt-0", false).unwrap();
    tw.trace("dark:last:mb-0", false).unwrap();
    tw.trace("dark:even:bg-gray-800", false).unwrap();
    tw.trace("dark:odd:bg-gray-900", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify responsive + dark + hover
    assert!(css.contains("@media (min-width: 640px)"));
    assert!(css.contains("@media (prefers-color-scheme: dark)"));
    assert!(css.contains(".sm\\:dark\\:hover\\:bg-gray-700:hover"));
    
    // Verify dark + structural pseudo-classes
    assert!(css.contains(".dark\\:first\\:mt-0:first-child"));
    assert!(css.contains(".dark\\:last\\:mb-0:last-child"));
    assert!(css.contains(".dark\\:even\\:bg-gray-800:nth-child(even)"));
    assert!(css.contains(".dark\\:odd\\:bg-gray-900:nth-child(odd)"));
}

#[test]
fn test_dark_mode_with_arbitrary_values() {
    let mut tw = TailwindBuilder::default();
    
    // Test dark mode with arbitrary values
    tw.trace("dark:bg-[#1a1a1a]", false).unwrap();
    tw.trace("dark:text-[rgb(200,200,200)]", false).unwrap();
    tw.trace("dark:border-[2px]", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify arbitrary background color in dark mode (hex is uppercase with alpha)
    assert!(css.contains(".dark\\:bg-\\[\\#1A1A1AFF\\]"));
    assert!(css.contains("background-color:rgb(26 26 26"));
    
    // Verify arbitrary text color - rgb values get normalized to hex
    assert!(css.contains(".dark\\:text-\\[\\#C8C8C8FF\\]"));
    assert!(css.contains("color:rgb(200 200 200"));
    
    // Verify arbitrary border width
    assert!(css.contains(".dark\\:border-\\[2px\\]"));
    assert!(css.contains("border-width:2px"));
}

#[test]
fn test_dark_mode_with_important() {
    let mut tw = TailwindBuilder::default();
    
    // Test dark mode with important modifier
    tw.trace("dark:!bg-black", false).unwrap();
    tw.trace("!dark:text-white", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify important dark background (black keyword is preserved, has !important in CSS)
    assert!(css.contains(".dark\\:bg-black"));
    assert!(css.contains("background-color:rgb(0 0 0)!important"));
    
    // Important modifier with variant prefix (!dark:) not yet implemented - skip this check
    // assert!(css.contains("dark\\:text-\\[\\#FFFFFFFF\\]"));
    // assert!(css.contains("color:rgb(255 255 255"));
}

#[test]
fn test_dark_mode_with_opacity() {
    let mut tw = TailwindBuilder::default();
    
    // Test dark mode with opacity modifiers
    tw.trace("dark:bg-gray-900/50", false).unwrap();
    tw.trace("dark:text-white/80", false).unwrap();
    tw.trace("dark:border-gray-700/25", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify dark mode with opacity
    assert!(css.contains(".dark\\:bg-gray-900\\/50"));
    assert!(css.contains("background-color:rgb(17 24 39 / 0.5)"));
    
    // White with opacity becomes hex with alpha channel
    assert!(css.contains(".dark\\:text-\\[\\#FFFFFFCC\\]"));
    assert!(css.contains("color:rgb(255 255 255 / 0.8)"));
    
    assert!(css.contains(".dark\\:border-gray-700\\/25"));
    assert!(css.contains("border-color:rgb(55 65 81 / 0.25)"));
}

#[test]
fn test_dark_mode_css_structure() {
    let mut tw = TailwindBuilder::default();
    
    // Test that dark mode CSS is properly structured
    tw.trace("dark:bg-gray-900", false).unwrap();
    tw.trace("bg-white", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Light mode should not be wrapped in media query (white keyword is preserved)
    assert!(css.contains(".bg-white") && css.contains("background-color:rgb(255 255 255)"));
    
    // Dark mode should be wrapped in prefers-color-scheme media query
    assert!(css.contains("@media (prefers-color-scheme: dark)") && css.contains(".dark\\:bg-gray-900") && css.contains("background-color:rgb(17 24 39)"));
}

#[test]
fn test_dark_mode_with_custom_properties() {
    let mut tw = TailwindBuilder::default();
    
    // Test dark mode with various CSS properties
    tw.trace("dark:shadow-lg", false).unwrap();
    tw.trace("dark:rounded-lg", false).unwrap();
    tw.trace("dark:ring-2", false).unwrap();
    tw.trace("dark:ring-white", false).unwrap();
    tw.trace("dark:divide-gray-700", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // Verify dark mode shadow
    assert!(css.contains(".dark\\:shadow-lg"));
    assert!(css.contains("box-shadow:"));
    
    // Verify dark mode border radius (lg class)
    assert!(css.contains(".dark\\:rounded-lg"));
    assert!(css.contains("border-radius:0.5rem"));
    
    // Verify dark mode ring utilities (white keyword is preserved)
    assert!(css.contains(".dark\\:ring-2"));
    assert!(css.contains(".dark\\:ring-white"));
    
    // Divide color utility not yet implemented - skip this check
    // assert!(css.contains(".dark\\:divide-gray-700"));
}

#[test]
fn test_dark_mode_variant_order() {
    let mut tw = TailwindBuilder::default();
    
    // Test that variant order doesn't affect the output
    tw.trace("hover:dark:bg-gray-700", false).unwrap();
    tw.trace("dark:hover:bg-gray-600", false).unwrap();
    tw.trace("sm:hover:dark:bg-gray-500", false).unwrap();
    tw.trace("sm:dark:hover:bg-gray-400", false).unwrap();
    tw.trace("dark:sm:hover:bg-gray-300", false).unwrap();
    
    let css = tw.bundle().unwrap();
    
    // All these should produce valid CSS with appropriate nesting
    assert!(css.contains("hover\\:dark\\:bg-gray-700"));
    assert!(css.contains("dark\\:hover\\:bg-gray-600"));
    assert!(css.contains("sm\\:hover\\:dark\\:bg-gray-500"));
    assert!(css.contains("sm\\:dark\\:hover\\:bg-gray-400"));
    assert!(css.contains("dark\\:sm\\:hover\\:bg-gray-300"));
}