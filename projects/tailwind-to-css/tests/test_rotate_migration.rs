use tailwind_css::TailwindBuilder;

#[test]
fn test_rotate_basic() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("rotate-45", false);
    
    let css = builder.bundle().unwrap();
    
    // Check for CSS variable
    assert!(css.contains("--tw-rotate:45deg"), "CSS should contain --tw-rotate CSS variable set to 45deg");
    assert!(css.contains("rotate(var(--tw-rotate"), "CSS should contain rotate() function with CSS variable");
    assert!(css.contains("transform:"), "CSS should contain transform property");
}

#[test]
fn test_rotate_negative() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("-rotate-12", false);
    
    let css = builder.bundle().unwrap();
    
    // Check for negative CSS variable
    assert!(css.contains("--tw-rotate:-12deg"), "CSS should contain negative rotation");
}

#[test]
fn test_rotate_arbitrary() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("rotate-[23deg]", false);
    
    let css = builder.bundle().unwrap();
    
    // Check for arbitrary value
    assert!(css.contains("--tw-rotate:23deg"), "CSS should contain arbitrary rotation value");
}

#[test]
fn test_rotate_zero() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("rotate-0", false);
    
    let css = builder.bundle().unwrap();
    
    // Check for zero rotation
    assert!(css.contains("--tw-rotate:0deg"), "CSS should contain 0deg rotation");
}

#[test]
fn test_rotate_with_modifier() {
    let mut builder = TailwindBuilder::default();
    let _ = builder.trace("hover:rotate-90", false);
    
    let css = builder.bundle().unwrap();
    
    // Check for hover modifier
    assert!(css.contains(".hover\\:rotate-90:hover"), "CSS should contain hover selector");
    assert!(css.contains("--tw-rotate:90deg"), "CSS should contain rotation value");
}