// Test file for layout edge case utilities
use tailwind_css::TailwindBuilder;

#[test]
fn test_visibility_utilities() {
    let mut builder = TailwindBuilder::default();
    
    // Test visible
    let result = builder.trace("visible", false);
    assert!(result.is_ok(), "Failed to parse 'visible'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("visibility:visible") || css.contains("visibility: visible"), "visible should generate visibility: visible");
    
    builder.clear();
    
    // Test invisible
    let result = builder.trace("invisible", false);
    assert!(result.is_ok(), "Failed to parse 'invisible'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("visibility:hidden") || css.contains("visibility: hidden"), "invisible should generate visibility: hidden");
}

#[test]
fn test_float_utilities() {
    let mut builder = TailwindBuilder::default();
    
    // Test float-left
    let result = builder.trace("float-left", false);
    assert!(result.is_ok(), "Failed to parse 'float-left'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("float:left") || css.contains("float: left"), "float-left should generate float: left");
    
    builder.clear();
    
    // Test float-right
    let result = builder.trace("float-right", false);
    assert!(result.is_ok(), "Failed to parse 'float-right'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("float:right") || css.contains("float: right"), "float-right should generate float: right");
    
    builder.clear();
    
    // Test float-none
    let result = builder.trace("float-none", false);
    assert!(result.is_ok(), "Failed to parse 'float-none'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("float:none") || css.contains("float: none"), "float-none should generate float: none");
}

#[test]
fn test_clear_utilities() {
    let mut builder = TailwindBuilder::default();
    
    // Test clear-left
    let result = builder.trace("clear-left", false);
    assert!(result.is_ok(), "Failed to parse 'clear-left'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("clear:left") || css.contains("clear: left"), "clear-left should generate clear: left");
    
    builder.clear();
    
    // Test clear-right
    let result = builder.trace("clear-right", false);
    assert!(result.is_ok(), "Failed to parse 'clear-right'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("clear:right") || css.contains("clear: right"), "clear-right should generate clear: right");
    
    builder.clear();
    
    // Test clear-both
    let result = builder.trace("clear-both", false);
    assert!(result.is_ok(), "Failed to parse 'clear-both'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("clear:both") || css.contains("clear: both"), "clear-both should generate clear: both");
    
    builder.clear();
    
    // Test clear-none
    let result = builder.trace("clear-none", false);
    assert!(result.is_ok(), "Failed to parse 'clear-none'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("clear:none") || css.contains("clear: none"), "clear-none should generate clear: none");
}

#[test]
fn test_isolation_utilities() {
    let mut builder = TailwindBuilder::default();
    
    // Test isolate
    let result = builder.trace("isolate", false);
    assert!(result.is_ok(), "Failed to parse 'isolate'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("isolation:isolate") || css.contains("isolation: isolate"), "isolate should generate isolation: isolate");
    
    builder.clear();
    
    // Test isolation-auto
    let result = builder.trace("isolation-auto", false);
    assert!(result.is_ok(), "Failed to parse 'isolation-auto'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("isolation:auto") || css.contains("isolation: auto"), "isolation-auto should generate isolation: auto");
}

#[test]
fn test_screen_reader_utilities() {
    let mut builder = TailwindBuilder::default();
    
    // Test sr-only
    let result = builder.trace("sr-only", false);
    assert!(result.is_ok(), "Failed to parse 'sr-only'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("position:absolute") || css.contains("position: absolute"), "sr-only should generate screen reader styles");
    assert!(css.contains("width:1px") || css.contains("width: 1px"), "sr-only should have 1px width");
    
    builder.clear();
    
    // Test not-sr-only
    let result = builder.trace("not-sr-only", false);
    assert!(result.is_ok(), "Failed to parse 'not-sr-only'");
    let _transformed = result.unwrap();
    let css = builder.bundle().unwrap();
    assert!(css.contains("position:static") || css.contains("position: static"), "not-sr-only should reset screen reader styles");
    assert!(css.contains("width:auto") || css.contains("width: auto"), "not-sr-only should have auto width");
}