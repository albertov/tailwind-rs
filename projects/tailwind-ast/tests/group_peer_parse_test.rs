use tailwind_ast::parse_tailwind;

#[test]
fn test_parse_group_hover() {
    let input = "group-hover:flex";
    let result = parse_tailwind(input).unwrap();
    
    assert_eq!(result.len(), 1);
    let style = &result[0];
    
    assert_eq!(style.variants.len(), 1);
    let variant = &style.variants[0];
    
    assert_eq!(variant.names, vec!["group", "hover"]);
    assert_eq!(variant.modifier, None);
    assert_eq!(style.elements, vec!["flex"]);
}

#[test]
fn test_parse_group_hover_with_modifier() {
    let input = "group-hover/sidebar:visible";
    let result = parse_tailwind(input).unwrap();
    
    assert_eq!(result.len(), 1);
    let style = &result[0];
    
    assert_eq!(style.variants.len(), 1);
    let variant = &style.variants[0];
    
    assert_eq!(variant.names, vec!["group", "hover"]);
    assert_eq!(variant.modifier, Some("sidebar"));
    assert_eq!(style.elements, vec!["visible"]);
}

#[test]
fn test_parse_peer_checked() {
    let input = "peer-checked:opacity-100";
    let result = parse_tailwind(input).unwrap();
    
    assert_eq!(result.len(), 1);
    let style = &result[0];
    
    assert_eq!(style.variants.len(), 1);
    let variant = &style.variants[0];
    
    assert_eq!(variant.names, vec!["peer", "checked"]);
    assert_eq!(variant.modifier, None);
    assert_eq!(style.elements, vec!["opacity", "100"]);
}

#[test]
fn test_parse_peer_focus_with_modifier() {
    let input = "peer-focus/input:border-red-500";
    let result = parse_tailwind(input).unwrap();
    
    assert_eq!(result.len(), 1);
    let style = &result[0];
    
    assert_eq!(style.variants.len(), 1);
    let variant = &style.variants[0];
    
    assert_eq!(variant.names, vec!["peer", "focus"]);
    assert_eq!(variant.modifier, Some("input"));
    assert_eq!(style.elements, vec!["border", "red", "500"]);
}

#[test]
fn test_parse_multiple_variants_with_group() {
    let input = "sm:group-hover:text-lg";
    let result = parse_tailwind(input).unwrap();
    
    assert_eq!(result.len(), 1);
    let style = &result[0];
    
    assert_eq!(style.variants.len(), 2);
    
    // First variant: sm
    assert_eq!(style.variants[0].names, vec!["sm"]);
    assert_eq!(style.variants[0].modifier, None);
    
    // Second variant: group-hover
    assert_eq!(style.variants[1].names, vec!["group", "hover"]);
    assert_eq!(style.variants[1].modifier, None);
    
    assert_eq!(style.elements, vec!["text", "lg"]);
}