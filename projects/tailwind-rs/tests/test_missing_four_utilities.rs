use tailwind_rs::TailwindBuilder;

#[test]
fn test_gap_arbitrary_rem_values() {
    let mut tw = TailwindBuilder::default();
    
    // Test gap-[0.25rem]
    let result = tw.trace("gap-[0.25rem]", false);
    assert!(result.is_ok(), "gap-[0.25rem] should be valid: {:?}", result);
    
    // Test gap-[1.5rem]
    let result = tw.trace("gap-[1.5rem]", false);
    assert!(result.is_ok(), "gap-[1.5rem] should be valid: {:?}", result);
    
    let css = tw.bundle().unwrap();
    println!("CSS for gap utilities:\n{}", css);
    assert!(css.contains("gap:0.25rem"), "gap-[0.25rem] should generate gap:0.25rem");
    assert!(css.contains("gap:1.5rem"), "gap-[1.5rem] should generate gap:1.5rem");
}

#[test]
fn test_leading_percentage_value() {
    let mut tw = TailwindBuilder::default();
    
    let result = tw.trace("leading-[162.5%]", false);
    assert!(result.is_ok(), "leading-[162.5%] should be valid: {:?}", result);
    
    let css = tw.bundle().unwrap();
    println!("CSS for leading utility:\n{}", css);
    assert!(css.contains("line-height:162.5%"), "leading-[162.5%] should generate line-height:162.5%");
}

#[test]
fn test_justify_between() {
    let mut tw = TailwindBuilder::default();
    
    let result = tw.trace("justify-between", false);
    println!("justify-between trace result: {:?}", result);
    assert!(result.is_ok(), "justify-between should be valid: {:?}", result);
    
    let css = tw.bundle().unwrap();
    println!("CSS for justify-between:\n{}", css);
    assert!(css.contains("justify-content:space-between"), "justify-between should generate justify-content:space-between");
}

#[test]
fn test_all_four_missing_utilities() {
    let mut tw = TailwindBuilder::default();
    
    // Trace all utilities
    let result1 = tw.trace("gap-[0.25rem]", false);
    assert!(result1.is_ok(), "gap-[0.25rem] should be valid: {:?}", result1);
    
    let result2 = tw.trace("gap-[1.5rem]", false);
    assert!(result2.is_ok(), "gap-[1.5rem] should be valid: {:?}", result2);
    
    let result3 = tw.trace("leading-[162.5%]", false);
    assert!(result3.is_ok(), "leading-[162.5%] should be valid: {:?}", result3);
    
    let result4 = tw.trace("justify-between", false);
    assert!(result4.is_ok(), "justify-between should be valid: {:?}", result4);
    
    let css = tw.bundle().unwrap();
    
    // Test gap utilities
    assert!(css.contains(".gap-\\[0\\.25rem\\]"), "CSS should contain gap-[0.25rem] selector");
    assert!(css.contains("gap:0.25rem"), "gap-[0.25rem] should generate gap:0.25rem");
    
    assert!(css.contains(".gap-\\[1\\.5rem\\]"), "CSS should contain gap-[1.5rem] selector");
    assert!(css.contains("gap:1.5rem"), "gap-[1.5rem] should generate gap:1.5rem");
    
    // Test leading utility
    assert!(css.contains(".leading-\\[162\\.5\\%\\]"), "CSS should contain leading-[162.5%] selector");
    assert!(css.contains("line-height:162.5%"), "leading-[162.5%] should generate line-height:162.5%");
    
    // Test justify-between utility
    assert!(css.contains(".justify-between"), "CSS should contain justify-between selector");
    assert!(css.contains("justify-content:space-between"), "justify-between should generate justify-content:space-between");
    
    println!("Generated CSS:\n{}", css);
}