#[cfg(test)]
mod important_basic_tests {
    use super::super::*;
    
    #[test]
    fn test_important_at_beginning() {
        // Test !flex (important at beginning)
        let input = "!flex";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse !flex");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set for !flex");
        assert_eq!(style.elements, vec!["flex"]);
    }
    
    #[test]
    fn test_important_at_end_still_works() {
        // Test flex! (important at end - original behavior)
        let input = "flex!";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse flex!");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set for flex!");
        assert_eq!(style.elements, vec!["flex"]);
    }
    
    #[test]
    fn test_important_with_variant() {
        // Test hover:!bg-blue-500
        let input = "hover:!bg-blue-500";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse hover:!bg-blue-500");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set");
        assert_eq!(style.variants.len(), 1);
        assert_eq!(style.variants[0].names, vec!["hover"]);
        assert_eq!(style.elements, vec!["bg", "blue", "500"]);
    }
    
    #[test]
    fn test_important_with_negative() {
        // Test !-mt-4
        let input = "!-mt-4";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse !-mt-4");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set");
        assert!(style.negative, "Negative flag should be set");
        assert_eq!(style.elements, vec!["mt", "4"]);
    }
    
    #[test]
    fn test_important_with_arbitrary() {
        // Test !bg-[#FF0000]
        let input = "!bg-[#FF0000]";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse !bg-[#FF0000]");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set");
        assert_eq!(style.elements, vec!["bg"]);
        assert_eq!(style.arbitrary, Some("#FF0000"));
    }
    
    #[test]
    fn test_important_with_opacity() {
        // Test !bg-red-500/50
        let input = "!bg-red-500/50";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse !bg-red-500/50");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set");
        assert_eq!(style.elements, vec!["bg", "red", "500"]);
        assert_eq!(style.opacity, Some("50"));
    }
    
    #[test]
    fn test_complex_important_chain() {
        // Test dark:hover:!bg-blue-500/75
        let input = "dark:hover:!bg-blue-500/75";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse dark:hover:!bg-blue-500/75");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set");
        assert_eq!(style.variants.len(), 2);
        assert_eq!(style.variants[0].names, vec!["dark"]);
        assert_eq!(style.variants[1].names, vec!["hover"]);
        assert_eq!(style.elements, vec!["bg", "blue", "500"]);
        assert_eq!(style.opacity, Some("75"));
    }
}