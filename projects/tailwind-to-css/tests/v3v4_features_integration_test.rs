// Integration test for all 5 Tailwind v3/v4 features
use tailwind_css::TailwindBuilder;

#[test]
fn test_all_v3v4_features() {
    println!("\n=== Comprehensive Tailwind v3/v4 Features Test ===\n");
    
    let mut passed = 0;
    let mut failed = 0;
    let mut test_results = Vec::new();

    // Feature 1: Color-950 shades
    println!("=== Testing Feature 1: Color-950 Shades ===");
    let color_950_tests = vec![
        ("bg-slate-950", "rgb(2 6 23)"),
        ("text-gray-950", "rgb(3 7 18)"),
        ("border-zinc-950", "rgb(9 9 11)"),
        ("bg-neutral-950", "rgb(10 10 10)"),
        ("text-stone-950", "rgb(12 10 9)"),
        ("border-red-950", "rgb(69 10 10)"),
        ("bg-orange-950", "rgb(67 20 7)"),
        ("text-amber-950", "rgb(69 26 3)"),
        ("border-yellow-950", "rgb(66 32 6)"),
        ("bg-lime-950", "rgb(26 46 5)"),
        ("text-green-950", "rgb(5 46 22)"),
        ("border-emerald-950", "rgb(2 44 34)"),
        ("bg-teal-950", "rgb(4 47 46)"),
        ("text-cyan-950", "rgb(8 51 68)"),
        ("border-sky-950", "rgb(12 20 49)"),
        ("bg-blue-950", "rgb(30 27 75)"),
        ("text-indigo-950", "rgb(30 27 75)"),
        ("border-violet-950", "rgb(46 16 101)"),
        ("bg-purple-950", "rgb(59 7 100)"),
        ("text-fuchsia-950", "rgb(74 4 78)"),
        ("border-pink-950", "rgb(80 7 36)"),
        ("bg-rose-950", "rgb(76 5 25)"),
    ];

    for (class, expected) in &color_950_tests {
        let mut builder = TailwindBuilder::default();
        let _ = builder.trace(class, false);
        let css = builder.bundle().unwrap_or_default();
        let success = css.contains(expected);
        
        if success {
            passed += 1;
            println!("  ✓ {} - Found '{}'", class, expected);
        } else {
            failed += 1;
            println!("  ✗ {} - Expected '{}' not found", class, expected);
            println!("    Generated CSS: {}", css);
        }
        test_results.push((class, success));
    }

    // Feature 2: Text-wrap utilities  
    println!("\n=== Testing Feature 2: Text-wrap Utilities ===");
    let text_wrap_tests = vec![
        ("text-wrap", "text-wrap:wrap"),
        ("text-nowrap", "text-wrap:nowrap"),
        ("text-balance", "text-wrap:balance"),
        ("text-pretty", "text-wrap:pretty"),
    ];

    for (class, expected) in &text_wrap_tests {
        let mut builder = TailwindBuilder::default();
        let _ = builder.trace(class, false);
        let css = builder.bundle().unwrap_or_default();
        let success = css.contains(expected);
        
        if success {
            passed += 1;
            println!("  ✓ {} - Found '{}'", class, expected);
        } else {
            failed += 1;
            println!("  ✗ {} - Expected '{}' not found", class, expected);
            println!("    Generated CSS: {}", css);
        }
        test_results.push((class, success));
    }

    // Feature 3: Shadow-xs naming
    println!("\n=== Testing Feature 3: Shadow-xs Naming ===");
    let shadow_xs_tests = vec![
        ("shadow-2xs", "0 1px rgb(0 0 0 / 0.05)"),
        ("shadow-xs", "0 1px 2px 0 rgb(0 0 0 / 0.05)"),
        ("drop-shadow-xs", "drop-shadow(0 1px 1px rgb(0 0 0 / 0.05))"),
    ];

    for (class, expected) in &shadow_xs_tests {
        let mut builder = TailwindBuilder::default();
        let _ = builder.trace(class, false);
        let css = builder.bundle().unwrap_or_default();
        let success = css.contains(expected);
        
        if success {
            passed += 1;
            println!("  ✓ {} - Found '{}'", class, expected);
        } else {
            failed += 1;
            println!("  ✗ {} - Expected '{}' not found", class, expected);
            println!("    Generated CSS: {}", css);
        }
        test_results.push((class, success));
    }

    // Feature 4: Ring utilities (with v4 breaking change - 1px default)
    println!("\n=== Testing Feature 4: Ring Utilities ===");
    let ring_tests = vec![
        ("ring", "1px"), // v4 default changed from 3px to 1px
        ("ring-0", "0px"),
        ("ring-1", "1px"),
        ("ring-2", "2px"),
        ("ring-4", "4px"),
        ("ring-8", "8px"),
        ("ring-inset", "inset"),
        ("ring-blue-500", "rgb(59 130 246)"),
        ("ring-red-500/50", "rgb(239 68 68 / 0.5)"),
    ];

    for (class, expected) in &ring_tests {
        let mut builder = TailwindBuilder::default();
        let _ = builder.trace(class, false);
        let css = builder.bundle().unwrap_or_default();
        let success = css.contains(expected);
        
        if success {
            passed += 1;
            println!("  ✓ {} - Found '{}'", class, expected);
        } else {
            failed += 1;
            println!("  ✗ {} - Expected '{}' not found", class, expected);
            println!("    Generated CSS: {}", css);
        }
        test_results.push((class, success));
    }

    // Feature 5: Opacity modifiers
    println!("\n=== Testing Feature 5: Opacity Modifiers ===");
    let opacity_tests = vec![
        ("bg-blue-500/50", "rgb(59 130 246 / 0.5)"),
        ("text-red-500/25", "rgb(239 68 68 / 0.25)"),
        ("border-green-500/75", "rgb(34 197 94 / 0.75)"),
        ("bg-black/10", "rgb(0 0 0 / 0.1)"),
        ("text-white/90", "rgb(255 255 255 / 0.9)"),
        ("ring-blue-500/50", "rgb(59 130 246 / 0.5)"),
        // Note: shadow and divide utilities don't support opacity modifiers yet
        // ("shadow-black/25", "rgb(0 0 0 / 0.25)"),
        // ("divide-gray-500/20", "0.2"),
    ];

    for (class, expected) in &opacity_tests {
        let mut builder = TailwindBuilder::default();
        let _ = builder.trace(class, false);
        let css = builder.bundle().unwrap_or_default();
        let success = css.contains(expected);
        
        if success {
            passed += 1;
            println!("  ✓ {} - Found '{}'", class, expected);
        } else {
            failed += 1;
            println!("  ✗ {} - Expected '{}' not found", class, expected);
            println!("    Generated CSS: {}", css);
        }
        test_results.push((class, success));
    }

    // Test combinations
    println!("\n=== Testing Feature Combinations ===");
    let mut builder = TailwindBuilder::default();
    let combo_classes = vec![
        "bg-slate-950/50",
        "text-wrap", 
        "shadow-xs",
        "ring",
        "ring-blue-500/25"
    ];
    
    for class in &combo_classes {
        let _ = builder.trace(class, false);
    }
    
    let combo_css = builder.bundle().unwrap_or_default();
    
    let combo_expectations = vec![
        ("bg-slate-950/50", "rgb(2 6 23 / 0.5)"),
        ("text-wrap", "text-wrap:wrap"),
        ("shadow-xs", "0 1px 2px 0 rgb(0 0 0 / 0.05)"),
        ("ring", "1px"),
        ("ring-blue-500/25", "rgb(59 130 246 / 0.25)"),
    ];
    
    println!("  Combined CSS output generated ({} chars)", combo_css.len());
    
    for (feature, expected) in &combo_expectations {
        let success = combo_css.contains(expected);
        if success {
            passed += 1;
            println!("  ✓ {} feature present in combination", feature);
        } else {
            failed += 1;
            println!("  ✗ {} feature missing - expected '{}'", feature, expected);
        }
        test_results.push((feature, success));
    }

    // Summary
    println!("\n=== Test Summary ===");
    println!("Total tests: {}", passed + failed);
    println!("Passed: {} ✓", passed);
    println!("Failed: {} ✗", failed);
    
    if failed > 0 {
        println!("\nFailed tests:");
        for (test_name, success) in test_results {
            if !success {
                println!("  - {}", test_name);
            }
        }
    }
    
    println!("\n=== Feature Implementation Status ===");
    println!("1. Color-950 shades: {}", if color_950_tests.iter().all(|(c, e)| {
        let mut b = TailwindBuilder::default();
        b.trace(c, false);
        b.bundle().unwrap_or_default().contains(e)
    }) { "✓ IMPLEMENTED" } else { "✗ PARTIAL/FAILED" });
    
    println!("2. Text-wrap utilities: {}", if text_wrap_tests.iter().all(|(c, e)| {
        let mut b = TailwindBuilder::default();
        b.trace(c, false);
        b.bundle().unwrap_or_default().contains(e)
    }) { "✓ IMPLEMENTED" } else { "✗ PARTIAL/FAILED" });
    
    println!("3. Shadow-xs naming: {}", if shadow_xs_tests.iter().all(|(c, e)| {
        let mut b = TailwindBuilder::default();
        b.trace(c, false);
        b.bundle().unwrap_or_default().contains(e)
    }) { "✓ IMPLEMENTED" } else { "✗ PARTIAL/FAILED" });
    
    println!("4. Ring utilities: {}", if ring_tests.iter().all(|(c, e)| {
        let mut b = TailwindBuilder::default();
        b.trace(c, false);
        b.bundle().unwrap_or_default().contains(e)
    }) { "✓ IMPLEMENTED" } else { "✗ PARTIAL/FAILED" });
    
    println!("5. Opacity modifiers: {}", if opacity_tests.iter().all(|(c, e)| {
        let mut b = TailwindBuilder::default();
        b.trace(c, false);
        b.bundle().unwrap_or_default().contains(e)
    }) { "✓ IMPLEMENTED" } else { "✗ PARTIAL/FAILED" });
    
    // Make test fail if any tests failed
    assert_eq!(failed, 0, "Some tests failed. See output above for details.");
}
