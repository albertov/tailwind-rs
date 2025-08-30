#[test]
fn ready() {
    println!("it works!");
}

#[test]
fn test_text_alignment_fixes() {
    use tailwind_css::TailwindTextAlignment;
    
    // Test text-center fix (should produce "text-center", not "font-align-center")
    let text_center = TailwindTextAlignment::from("center");
    let text_center_output = text_center.to_string();
    println!("text-center produces: '{}'", text_center_output);
    assert_eq!(text_center_output, "text-center", "text-center should produce 'text-center'");
    
    // Test text-left fix
    let text_left = TailwindTextAlignment::from("left");
    let text_left_output = text_left.to_string();
    println!("text-left produces: '{}'", text_left_output);
    assert_eq!(text_left_output, "text-left", "text-left should produce 'text-left'");
    
    // Test text-right fix
    let text_right = TailwindTextAlignment::from("right");
    let text_right_output = text_right.to_string();
    println!("text-right produces: '{}'", text_right_output);
    assert_eq!(text_right_output, "text-right", "text-right should produce 'text-right'");
    
    // Test text-justify fix
    let text_justify = TailwindTextAlignment::from("justify");
    let text_justify_output = text_justify.to_string();
    println!("text-justify produces: '{}'", text_justify_output);
    assert_eq!(text_justify_output, "text-justify", "text-justify should produce 'text-justify'");
}

#[test]
fn test_transition_fixes() {
    use tailwind_css::{TailwindTransition, TailwindArbitrary};
    
    let empty_arbitrary = TailwindArbitrary::from("");
    
    // Test transition-colors fix (should produce "transition-colors", not "transition[-colors]")
    let transition_colors = TailwindTransition::parse(&["colors"], &empty_arbitrary).unwrap();
    let transition_output = transition_colors.to_string();
    println!("transition-colors produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition-colors", "transition-colors should produce 'transition-colors'");
    
    // Test transition-opacity fix
    let transition_opacity = TailwindTransition::parse(&["opacity"], &empty_arbitrary).unwrap();
    let transition_output = transition_opacity.to_string();
    println!("transition-opacity produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition-opacity", "transition-opacity should produce 'transition-opacity'");
    
    // Test basic transition (should produce "transition")
    let transition_basic = TailwindTransition::parse(&[], &empty_arbitrary).unwrap();
    let transition_output = transition_basic.to_string();
    println!("transition produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition", "basic transition should produce 'transition'");
    
    // Test transition-all
    let transition_all = TailwindTransition::parse(&["all"], &empty_arbitrary).unwrap();
    let transition_output = transition_all.to_string();
    println!("transition-all produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition-all", "transition-all should produce 'transition-all'");
    
    // Test transition-transform
    let transition_transform = TailwindTransition::parse(&["transform"], &empty_arbitrary).unwrap();
    let transition_output = transition_transform.to_string();
    println!("transition-transform produces: '{}'", transition_output);
    assert_eq!(transition_output, "transition-transform", "transition-transform should produce 'transition-transform'");
}
