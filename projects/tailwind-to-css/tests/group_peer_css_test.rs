use tailwind_css::TailwindBuilder;

#[test]
fn test_group_hover_css_generation() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("group-hover:flex", false);
    
    let css = builder.bundle().unwrap();
    
    println!("Generated CSS for group-hover:flex:\n{}", css);
    
    // Check the CSS contains correct selector pattern
    assert!(css.contains(".group-hover\\:flex"), "CSS should contain escaped class name");
    assert!(css.contains(".group:hover .group-hover\\:flex"), "CSS should contain correct group hover selector pattern");
    assert!(css.contains("display:flex"), "CSS should contain flex display");
}

#[test]
fn test_peer_checked_css_generation() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("peer-checked:opacity-100", false);
    
    let css = builder.bundle().unwrap();
    
    println!("Generated CSS for peer-checked:opacity-100:\n{}", css);
    
    // Check the CSS contains correct selector pattern
    assert!(css.contains(".peer-checked\\:opacity-100"), "CSS should contain escaped class name");
    assert!(css.contains(".peer:checked ~ .peer-checked\\:opacity-100"), "CSS should contain correct peer checked selector pattern");
    assert!(css.contains("opacity:100%"), "CSS should contain opacity property");
}

#[test]
fn test_group_hover_named_css_generation() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("group-hover/sidebar:visible", false);
    
    let css = builder.bundle().unwrap();
    
    println!("Generated CSS for group-hover/sidebar:visible:\n{}", css);
    
    // Check the CSS contains correct selector pattern with named group
    assert!(css.contains(".group-hover\\/sidebar\\:visible"), "CSS should contain escaped class name with modifier");
    assert!(css.contains(".group\\/sidebar:hover .group-hover\\/sidebar\\:visible"), "CSS should contain correct named group hover selector pattern");
    assert!(css.contains("visibility:visible"), "CSS should contain visibility property");
}

#[test]
fn test_peer_focus_named_css_generation() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("peer-focus/input:border-red-500", false);
    
    let css = builder.bundle().unwrap();
    
    println!("Generated CSS for peer-focus/input:border-red-500:\n{}", css);
    
    // Check the CSS contains correct selector pattern with named peer
    assert!(css.contains(".peer-focus\\/input\\:border-red-500"), "CSS should contain escaped class name with modifier");
    assert!(css.contains(".peer\\/input:focus ~ .peer-focus\\/input\\:border-red-500"), "CSS should contain correct named peer focus selector pattern");
    // Border color property check will depend on color implementation
}

#[test]
fn test_group_hover_media_query() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("group-hover:text-blue-500", false);
    
    let css = builder.bundle().unwrap();
    
    println!("Generated CSS for group-hover:text-blue-500:\n{}", css);
    
    // Check hover is wrapped in media query
    assert!(css.contains("@media (hover: hover)"), "CSS should contain hover media query");
    assert!(css.contains(".group-hover\\:text-blue-500"), "CSS should contain escaped class name");
}

#[test]
fn test_combined_responsive_group() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("sm:group-hover:text-lg", false);
    
    let css = builder.bundle().unwrap();
    
    println!("Generated CSS for sm:group-hover:text-lg:\n{}", css);
    
    // Check both responsive and group hover work together
    assert!(css.contains("@media (min-width: 640px)"), "CSS should contain responsive media query");
    assert!(css.contains("@media (hover: hover)"), "CSS should contain hover media query");
    assert!(css.contains(".sm\\:group-hover\\:text-lg"), "CSS should contain escaped class name");
    assert!(css.contains(".group:hover .sm\\:group-hover\\:text-lg"), "CSS should contain correct group hover selector pattern");
}