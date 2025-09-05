/// Test that demonstrates the whitespace bug in trace() function
/// Bug: Space between "transition-colors" and "text-gray-600" is being removed
use tailwind_css::TailwindBuilder;

#[test]
fn test_trace_preserves_whitespace_after_transition_colors() {
    let mut builder = TailwindBuilder::default();
    
    // The exact input string that demonstrates the bug
    let input = "px-4 py-2 rounded-lg font-[500] transition-colors text-gray-600 hover:bg-gray-100";
    
    // Call trace directly (not through HTML processing)
    let result = builder.trace(input, false).unwrap();
    
    // The bug causes "transition-colorstext-gray-600" (missing space)
    // This assertion should FAIL with the current buggy code
    assert!(
        result.contains("transition-colors text-gray-600"),
        "Space should be preserved between 'transition-colors' and 'text-gray-600'. Got: '{}'",
        result
    );
    
    // Additional check: ensure no classes are concatenated without spaces
    assert!(
        !result.contains("transition-colorstext"),
        "Classes should not be concatenated. Found 'transition-colorstext' in: '{}'", 
        result
    );
}

#[test]
fn test_trace_preserves_all_whitespace() {
    let mut builder = TailwindBuilder::default();
    
    // Test various class combinations
    let test_cases = vec![
        "transition-colors text-gray-600",
        "transition-all text-blue-500",
        "transition-opacity text-red-400", 
        "transition-transform text-green-300",
        "transition-shadow text-purple-200",
        "font-bold transition-colors text-gray-600",
        "px-4 transition-colors text-gray-600 py-2",
        " px-4 transition-colors text-gray-600 py-2",
        "px-4 transition-colors text-gray-600 py-2 ",
        " px-4 transition-colors text-gray-600 py-2 ",
    ];
    
    for input in test_cases {
        let result = builder.trace(input, false).unwrap();
        
        // Count spaces in input and result
        let input_spaces = input.chars().filter(|&c| c == ' ').count();
        let result_spaces = result.chars().filter(|&c| c == ' ').count();
        
        assert!(
            result_spaces >= input_spaces, // >= because some classes might be expanded 
            "Whitespace count mismatch for '{}'. Input has {} spaces, result has {} spaces. Result: '{}'",
            input, input_spaces, result_spaces, result
        );
        
        // Check specific known problematic pattern
        if input.contains("transition-colors text-") {
            assert!(
                result.contains("transition-colors text-") || result.contains("transition-colors text-"),
                "Space after transition-colors should be preserved. Input: '{}', Result: '{}'",
                input, result
            );
        }
    }
}

#[test]
fn test_trace_with_arbitrary_values_and_spaces() {
    let mut builder = TailwindBuilder::default();
    
    // Test that arbitrary values don't mess up spacing
    let input = "font-[500] transition-colors text-gray-600";
    let result = builder.trace(input, false).unwrap();
    
    // Should preserve spaces between all three classes
    assert!(
        !result.contains("font-[500]transition"),
        "Space should be preserved after font-[500]. Got: '{}'",
        result
    );
    
    assert!(
        !result.contains("transition-colorstext"),
        "Space should be preserved after transition-colors. Got: '{}'", 
        result
    );
}
