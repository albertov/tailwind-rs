#[test]
fn test_oklch_arbitrary_values() {
    use tailwind_css::TailwindBuilder;
    
    let mut builder = TailwindBuilder::default();
    
    // Test OKLCH color space with underscores for spaces
    let tests = vec![
        ("bg-[oklch(0.7_0.3_150)]", "background-color: oklch(0.7 0.3 150)"),
        ("text-[oklch(0.5_0.2_90)]", "color: oklch(0.5 0.2 90)"),
        ("border-[oklch(0.8_0.1_270)]", "border-color: oklch(0.8 0.1 270)"),
        ("bg-[oklch(0.7_0.3_150_/_0.5)]", "background-color: oklch(0.7 0.3 150 / 0.5)"),
    ];
    
    for (input, expected_css) in tests {
        let (_, css) = builder.inline(input).unwrap();
        assert!(
            css.contains(expected_css) || css.contains(&expected_css.replace(": ", ":")),
            "Failed for {}: expected '{}' in result '{}'",
            input,
            expected_css,
            css
        );
    }
}

#[test]
fn test_lch_arbitrary_values() {
    use tailwind_css::TailwindBuilder;
    
    let mut builder = TailwindBuilder::default();
    
    // Test LCH color space
    let tests = vec![
        ("bg-[lch(50%_100_150)]", "background-color: lch(50% 100 150)"),
        ("text-[lch(70%_50_90)]", "color: lch(70% 50 90)"),
        ("border-[lch(30%_80_270)]", "border-color: lch(30% 80 270)"),
        ("bg-[lch(50%_100_150_/_0.8)]", "background-color: lch(50% 100 150 / 0.8)"),
    ];
    
    for (input, expected_css) in tests {
        let (_, css) = builder.inline(input).unwrap();
        assert!(
            css.contains(expected_css) || css.contains(&expected_css.replace(": ", ":")),
            "Failed for {}: expected '{}' in result '{}'",
            input,
            expected_css,
            css
        );
    }
}

#[test]
fn test_display_p3_arbitrary_values() {
    use tailwind_css::TailwindBuilder;
    
    let mut builder = TailwindBuilder::default();
    
    // Test Display-P3 color space
    let tests = vec![
        ("bg-[color(display-p3_1_0_0)]", "background-color: color(display-p3 1 0 0)"),
        ("text-[color(display-p3_0_1_0)]", "color: color(display-p3 0 1 0)"),
        ("border-[color(display-p3_0_0_1)]", "border-color: color(display-p3 0 0 1)"),
        ("bg-[color(display-p3_1_0.5_0_/_0.7)]", "background-color: color(display-p3 1 0.5 0 / 0.7)"),
    ];
    
    for (input, expected_css) in tests {
        let (_, css) = builder.inline(input).unwrap();
        assert!(
            css.contains(expected_css) || css.contains(&expected_css.replace(": ", ":")),
            "Failed for {}: expected '{}' in result '{}'",
            input,
            expected_css,
            css
        );
    }
}

#[test]
fn test_lab_arbitrary_values() {
    use tailwind_css::TailwindBuilder;
    
    let mut builder = TailwindBuilder::default();
    
    // Test LAB color space
    let tests = vec![
        ("bg-[lab(50%_50_50)]", "background-color: lab(50% 50 50)"),
        ("text-[lab(70%_-50_30)]", "color: lab(70% -50 30)"),
        ("border-[lab(30%_80_-20)]", "border-color: lab(30% 80 -20)"),
    ];
    
    for (input, expected_css) in tests {
        let (_, css) = builder.inline(input).unwrap();
        assert!(
            css.contains(expected_css) || css.contains(&expected_css.replace(": ", ":")),
            "Failed for {}: expected '{}' in result '{}'",
            input,
            expected_css,
            css
        );
    }
}

#[test]
fn test_oklab_arbitrary_values() {
    use tailwind_css::TailwindBuilder;
    
    let mut builder = TailwindBuilder::default();
    
    // Test OKLAB color space
    let tests = vec![
        ("bg-[oklab(0.5_0.1_0.2)]", "background-color: oklab(0.5 0.1 0.2)"),
        ("text-[oklab(0.7_-0.1_0.1)]", "color: oklab(0.7 -0.1 0.1)"),
        ("border-[oklab(0.3_0.2_-0.1)]", "border-color: oklab(0.3 0.2 -0.1)"),
    ];
    
    for (input, expected_css) in tests {
        let (_, css) = builder.inline(input).unwrap();
        assert!(
            css.contains(expected_css) || css.contains(&expected_css.replace(": ", ":")),
            "Failed for {}: expected '{}' in result '{}'",
            input,
            expected_css,
            css
        );
    }
}

#[test]
fn test_hwb_arbitrary_values() {
    use tailwind_css::TailwindBuilder;
    
    let mut builder = TailwindBuilder::default();
    
    // Test HWB color space
    let tests = vec![
        ("bg-[hwb(150_20%_10%)]", "background-color: hwb(150 20% 10%)"),
        ("text-[hwb(90_10%_5%)]", "color: hwb(90 10% 5%)"),
        ("border-[hwb(270_30%_20%)]", "border-color: hwb(270 30% 20%)"),
        ("bg-[hwb(150_20%_10%_/_0.5)]", "background-color: hwb(150 20% 10% / 0.5)"),
    ];
    
    for (input, expected_css) in tests {
        let (_, css) = builder.inline(input).unwrap();
        assert!(
            css.contains(expected_css) || css.contains(&expected_css.replace(": ", ":")),
            "Failed for {}: expected '{}' in result '{}'",
            input,
            expected_css,
            css
        );
    }
}

#[test]
fn test_rec2020_arbitrary_values() {
    use tailwind_css::TailwindBuilder;
    
    let mut builder = TailwindBuilder::default();
    
    // Test Rec2020 color space
    let tests = vec![
        ("bg-[color(rec2020_1_0_0)]", "background-color: color(rec2020 1 0 0)"),
        ("text-[color(rec2020_0_1_0)]", "color: color(rec2020 0 1 0)"),
        ("border-[color(rec2020_0_0_1)]", "border-color: color(rec2020 0 0 1)"),
    ];
    
    for (input, expected_css) in tests {
        let (_, css) = builder.inline(input).unwrap();
        assert!(
            css.contains(expected_css) || css.contains(&expected_css.replace(": ", ":")),
            "Failed for {}: expected '{}' in result '{}'",
            input,
            expected_css,
            css
        );
    }
}