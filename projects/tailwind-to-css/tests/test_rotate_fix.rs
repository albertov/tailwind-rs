use tailwind_css::TailwindBuilder;

#[test]
fn test_group_hover_rotate_12() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("group-hover:rotate-12", false);
    
    let css = builder.bundle().unwrap();
    
    println!("Generated CSS for group-hover:rotate-12:");
    for line in css.lines() {
        if line.contains("rotate") && line.contains("group") {
            println!("{}", line);
        }
    }
    
    // Check the CSS contains correct selector pattern
    assert!(css.contains(".group-hover\\:rotate-12"), "CSS should contain escaped class name");
    assert!(css.contains(".group:hover .group-hover\\:rotate-12"), "CSS should contain correct group hover selector pattern");
    
    // Check for the CSS variable and composite transform pattern used by Tailwind
    assert!(css.contains("--tw-rotate:12deg"), "CSS should contain --tw-rotate CSS variable set to 12deg");
    assert!(css.contains("rotate(var(--tw-rotate"), "CSS should contain rotate() function with CSS variable");
}