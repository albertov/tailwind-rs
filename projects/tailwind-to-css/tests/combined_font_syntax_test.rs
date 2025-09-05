use tailwind_css::TailwindBuilder;

#[test]
fn test_combined_font_syntax_with_numeric_line_height() {
    let mut ctx = TailwindBuilder::default();
    
    // Test text-sm/6
    ctx.trace("text-sm/6", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("0.875rem")); // sm = 0.875rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("1.5rem")); // 6 * 0.25rem = 1.5rem
    
    // Test text-base/7
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-base/7", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1rem")); // base = 1rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("1.75rem")); // 7 * 0.25rem = 1.75rem
    
    // Test text-lg/5
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-lg/5", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1.125rem")); // lg = 1.125rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("1.25rem")); // 5 * 0.25rem = 1.25rem
    
    // Test text-2xl/10
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-2xl/10", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1.5rem")); // 2xl = 1.5rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("2.5rem")); // 10 * 0.25rem = 2.5rem
}

#[test]
fn test_combined_font_syntax_with_keyword_line_height() {
    let mut ctx = TailwindBuilder::default();
    
    // Test text-sm/tight
    ctx.trace("text-sm/tight", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("0.875rem")); // sm = 0.875rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("125%")); // tight = 1.25
    
    // Test text-lg/loose
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-lg/loose", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1.125rem")); // lg = 1.125rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("200%")); // loose = 2
    
    // Test text-base/normal
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-base/normal", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1rem")); // base = 1rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("normal")); // normal = normal
    
    // Test text-xl/snug
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-xl/snug", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1.25rem")); // xl = 1.25rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("137.5%")); // snug = 1.375
    
    // Test text-2xl/relaxed
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-2xl/relaxed", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1.5rem")); // 2xl = 1.5rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("162.5%")); // relaxed = 1.625
}

#[test]
fn test_combined_font_syntax_with_arbitrary_line_height() {
    let mut ctx = TailwindBuilder::default();
    
    // Test text-sm/[20px]
    ctx.trace("text-sm/[20px]", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("0.875rem")); // sm = 0.875rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("20px"));
    
    // Test text-lg/[2]
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-lg/[2]", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1.125rem")); // lg = 1.125rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("2"));
    
    // Test text-base/[1.8rem]
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-base/[1.8rem]", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1rem")); // base = 1rem
    assert!(css.contains("line-height:"));
    assert!(css.contains("1.8rem"));
}

#[test]
fn test_arbitrary_font_size_with_line_height() {
    let mut ctx = TailwindBuilder::default();
    
    // Test text-[14px]/6
    ctx.trace("text-[14px]/6", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("14px"));
    assert!(css.contains("line-height:"));
    assert!(css.contains("1.5rem")); // 6 * 0.25rem = 1.5rem
    
    // Test text-[1.5rem]/tight
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-[1.5rem]/tight", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("1.5rem"));
    assert!(css.contains("line-height:"));
    assert!(css.contains("125%")); // tight = 1.25
    
    // Test text-[20px]/[30px]
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-[20px]/[30px]", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("20px"));
    assert!(css.contains("line-height:"));
    assert!(css.contains("30px"));
}

#[test]
fn test_backward_compatibility_text_utilities() {
    let mut ctx = TailwindBuilder::default();
    
    // Test that regular text-size still works
    ctx.trace("text-sm", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("font-size:"));
    assert!(css.contains("0.875rem"));
    assert!(css.contains("line-height:")); // Should include default line-height
    
    // Test that text-color still works
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-red-500", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("color:"));
    assert!(css.contains("rgb(239"));
    
    // Test text color with opacity
    let mut ctx = TailwindBuilder::default();
    ctx.trace("text-blue-500/50", false).unwrap();
    let css = ctx.bundle().unwrap();
    assert!(css.contains("color:"));
    assert!(css.contains("rgb"));
    assert!(css.contains("0.5")); // 50% opacity
}

#[test]
fn test_all_numeric_line_heights() {
    let test_cases = vec![
        ("text-base/3", "0.75rem"),
        ("text-base/4", "1rem"),
        ("text-base/5", "1.25rem"),
        ("text-base/6", "1.5rem"),
        ("text-base/7", "1.75rem"),
        ("text-base/8", "2rem"),
        ("text-base/9", "2.25rem"),
        ("text-base/10", "2.5rem"),
    ];
    
    for (input, expected_line_height) in test_cases {
        let mut ctx = TailwindBuilder::default();
        ctx.trace(input, false).unwrap();
        let css = ctx.bundle().unwrap();
        assert!(css.contains("font-size:"));
        assert!(css.contains("1rem")); // base = 1rem
        assert!(css.contains("line-height:"));
        assert!(css.contains(expected_line_height), "Failed for {}: expected line-height {}", input, expected_line_height);
    }
}

#[test]
fn test_all_keyword_line_heights() {
    let test_cases = vec![
        ("text-base/none", "100%"),
        ("text-base/tight", "125%"),
        ("text-base/snug", "137.5%"),
        ("text-base/normal", "normal"),
        ("text-base/relaxed", "162.5%"),
        ("text-base/loose", "200%"),
    ];
    
    for (input, expected_line_height) in test_cases {
        let mut ctx = TailwindBuilder::default();
        ctx.trace(input, false).unwrap();
        let css = ctx.bundle().unwrap();
        assert!(css.contains("font-size:"));
        assert!(css.contains("1rem")); // base = 1rem
        assert!(css.contains("line-height:"));
        assert!(css.contains(expected_line_height), "Failed for {}: expected line-height {}", input, expected_line_height);
    }
}