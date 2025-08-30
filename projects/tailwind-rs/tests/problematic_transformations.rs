/// Tests for problematic transformations found in react_ssr integration
/// These tests document issues where Tailwind CSS classes are being incorrectly transformed
/// by the tailwind-rs library's trace() functionality.
use tailwind_css::{CssInlineMode, TailwindBuilder};
use tailwind_rs::CLIConfig;

fn pre_config() -> (CLIConfig, TailwindBuilder) {
    let mut config = CLIConfig::default();
    let mut builder = config.builder();
    config.minify = false;
    builder.preflight.disable = true;
    (config, builder)
}

#[test]
fn test_font_weight_transformations() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    
    let html = r#"
    <div class="font-bold">Bold text</div>
    <div class="font-medium">Medium text</div>
    <div class="font-semibold">Semibold text</div>
    <div class="font-normal">Normal text</div>
    <div class="font-light">Light text</div>
    "#;
    
    let (output_html, css) = config.compile_html(html, &mut builder).unwrap();
    
    // Check that font-bold transforms to font-[700]
    assert!(output_html.contains(r#"class="font-[700]""#), 
            "font-bold should transform to font-[700], got: {}", output_html);
    
    // Check that font-medium transforms to font-[500]
    assert!(output_html.contains(r#"class="font-[500]""#), 
            "font-medium should transform to font-[500], got: {}", output_html);
    
    // Check that font-semibold transforms to font-[600]
    assert!(output_html.contains(r#"class="font-[600]""#), 
            "font-semibold should transform to font-[600], got: {}", output_html);
    
    // Check CSS contains correct font-weight values
    assert!(css.contains("font-weight: 700"), 
            "CSS should contain font-weight: 700 for font-bold");
    assert!(css.contains("font-weight: 500"), 
            "CSS should contain font-weight: 500 for font-medium");
}

#[test]
fn test_text_alignment_transformations() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    
    let html = r#"
    <div class="text-center">Centered text</div>
    <div class="text-left">Left aligned text</div>
    <div class="text-right">Right aligned text</div>
    <div class="text-justify">Justified text</div>
    "#;
    
    let (output_html, css) = config.compile_html(html, &mut builder).unwrap();
    
    // FIXED: text-center now stays as text-center (correct)
    assert!(output_html.contains(r#"class="text-center""#), 
            "text-center should stay as text-center, got: {}", output_html);
    
    assert!(output_html.contains(r#"class="text-left""#), 
            "text-left should stay as text-left, got: {}", output_html);
    
    assert!(output_html.contains(r#"class="text-right""#), 
            "text-right should stay as text-right, got: {}", output_html);
    
    assert!(output_html.contains(r#"class="text-justify""#), 
            "text-justify should stay as text-justify, got: {}", output_html);
    
    // Check CSS contains text-align properties (this part is actually correct)
    assert!(css.contains("text-align: center"), 
            "CSS correctly contains text-align: center");
    assert!(css.contains("text-align: left"), 
            "CSS correctly contains text-align: left");
}

#[test]
fn test_transition_transformations() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    
    let html = r#"
    <div class="transition">Default transition</div>
    <div class="transition-none">No transition</div>
    <div class="transition-all">All transition</div>
    <div class="transition-colors">Colors transition</div>
    <div class="transition-opacity">Opacity transition</div>
    <div class="transition-shadow">Shadow transition</div>
    <div class="transition-transform">Transform transition</div>
    "#;
    
    let (output_html, _css) = config.compile_html(html, &mut builder).unwrap();
    
    // FIXED: transition classes now format correctly
    assert!(output_html.contains(r#"class="transition""#), 
            "transition should stay as transition, got: {}", output_html);
    
    assert!(output_html.contains(r#"class="transition-none""#), 
            "transition-none should stay as transition-none, got: {}", output_html);
    
    assert!(output_html.contains(r#"class="transition-all""#), 
            "transition-all should stay as transition-all, got: {}", output_html);
    
    assert!(output_html.contains(r#"class="transition-colors""#), 
            "transition-colors should stay as transition-colors, got: {}", output_html);
    
    assert!(output_html.contains(r#"class="transition-opacity""#), 
            "transition-opacity should stay as transition-opacity, got: {}", output_html);
    
    assert!(output_html.contains(r#"class="transition-shadow""#), 
            "transition-shadow should stay as transition-shadow, got: {}", output_html);
    
    assert!(output_html.contains(r#"class="transition-transform""#), 
            "transition-transform should stay as transition-transform, got: {}", output_html);
}

#[test]
fn test_transition_arbitrary_values() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    
    let html = r#"
    <div class="transition-[opacity]">Single property transition</div>
    <div class="transition-[opacity,width]">Multiple properties transition</div>
    <div class="transition-[opacity,width,transform]">Three properties transition</div>
    "#;
    
    let (output_html, _css) = config.compile_html(html, &mut builder).unwrap();
    
    // Check single property - should have single brackets
    assert!(output_html.contains(r#"class="transition-[opacity]""#), 
            "transition-[opacity] should stay as transition-[opacity] with single brackets, got: {}", output_html);
    
    // Check multiple properties - should have single brackets
    assert!(output_html.contains(r#"class="transition-[opacity,width]""#), 
            "transition-[opacity,width] should stay as transition-[opacity,width] with single brackets, got: {}", output_html);
    
    // Check three properties - should have single brackets
    assert!(output_html.contains(r#"class="transition-[opacity,width,transform]""#), 
            "transition-[opacity,width,transform] should stay as transition-[opacity,width,transform] with single brackets, got: {}", output_html);
    
    // Ensure no double brackets
    assert!(!output_html.contains("[["), 
            "Output should not contain double brackets [[, got: {}", output_html);
    assert!(!output_html.contains("]]"), 
            "Output should not contain double brackets ]], got: {}", output_html);
}

#[test]
fn test_truncation_issue() {
    let (mut config, mut builder) = pre_config();
    config.mode = CssInlineMode::None;
    
    // Testing a specific case where font-[700] might get truncated to font-
    let html = r#"<div class="font-bold hover:font-medium">Text with font weights</div>"#;
    
    let (output_html, css) = config.compile_html(html, &mut builder).unwrap();
    
    // Check that the transformation doesn't truncate to just "font-"
    assert!(!output_html.contains(r#"class="font- "#), 
            "font-[700] should not be truncated to 'font-'");
    
    // Should contain the full transformed classes
    assert!(output_html.contains("font-[700]") || output_html.contains("hover:font-[500]"), 
            "Should contain full transformed classes, got: {}", output_html);
    
    // CSS should be valid
    assert!(css.contains("font-weight"), 
            "CSS should contain font-weight properties");
}

/// Summary of issues FIXED:
/// 1. ✅ FIXED: text-center now stays as text-center (was: font-align-center)
/// 2. ✅ FIXED: text-left now stays as text-left (was: font-align-left)
/// 3. ✅ FIXED: text-right now stays as text-right (was: font-align-right)
/// 4. ✅ FIXED: text-justify now stays as text-justify (was: font-align-justify)
/// 5. ✅ FIXED: transition now stays as transition (was: transition[])
/// 6. ✅ FIXED: transition-colors now stays as transition-colors (was: transition[-colors])
/// 7. ✅ FIXED: All transition-* classes now format correctly
/// 8. ✅ FIXED: transition-[opacity,width] now stays with single brackets (was: transition-[[opacity,width]])
/// 
/// Issues that remain (but are technically correct transformations):
/// - font-bold transforms to font-[700]
/// - font-medium transforms to font-[500]
/// - font-semibold transforms to font-[600]
#[test]
fn document_all_issues() {
    // This test documents the fixes applied
    println!("=== TAILWIND-RS TRANSFORMATION FIXES APPLIED ===");
    println!("✅ FIXED Text alignment classes:");
    println!("   text-center now stays as text-center (was: font-align-center)");
    println!("   text-left now stays as text-left (was: font-align-left)");
    println!("   text-right now stays as text-right (was: font-align-right)");
    println!("   text-justify now stays as text-justify (was: font-align-justify)");
    println!();
    println!("✅ FIXED Transition classes:");
    println!("   transition now stays as transition (was: transition[])");
    println!("   transition-colors now stays as transition-colors (was: transition[-colors])");
    println!("   transition-opacity now stays as transition-opacity (was: transition[-opacity])");
    println!("   transition-[opacity,width] now has single brackets (was: transition-[[opacity,width]])");
    println!("   All transition-* classes now format correctly");
    println!();
    println!("ℹ️ Font weight transformations (technically correct but may need review):");
    println!("   font-bold -> font-[700]");
    println!("   font-medium -> font-[500]");
    println!("   font-semibold -> font-[600]");
    println!("   These transformations are technically correct per Tailwind CSS spec");
}