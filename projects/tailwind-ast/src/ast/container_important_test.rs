#[cfg(test)]
mod container_important_tests {
    use super::super::*;
    
    #[test]
    fn test_parse_container_query_with_important() {
        // Test @lg:!flex
        let input = "@lg:!flex";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse @lg:!flex");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert_eq!(style.variants.len(), 1);
        assert!(style.variants[0].container);
        assert_eq!(style.variants[0].container_type, Some(ContainerQueryType::Min));
        assert_eq!(style.variants[0].names, vec!["lg"]);
        assert!(style.important, "Important flag should be set for @lg:!flex");
        assert_eq!(style.elements, vec!["flex"]);
    }
    
    #[test]
    fn test_parse_container_query_max_with_important() {
        // Test @max-lg:!hidden
        let input = "@max-lg:!hidden";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse @max-lg:!hidden");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set for @max-lg:!hidden");
        assert_eq!(style.elements, vec!["hidden"]);
    }
    
    #[test]
    fn test_parse_container_query_arbitrary_with_important() {
        // Test @[300px]:!block
        let input = "@[300px]:!block";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse @[300px]:!block");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set for @[300px]:!block");
        assert_eq!(style.elements, vec!["block"]);
    }
    
    #[test]
    fn test_parse_container_query_named_with_important() {
        // Test @lg/sidebar:!p-4
        let input = "@lg/sidebar:!p-4";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse @lg/sidebar:!p-4");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set for @lg/sidebar:!p-4");
        assert_eq!(style.elements, vec!["p", "4"]);
        assert_eq!(style.variants[0].modifier, Some("sidebar"));
    }
    
    #[test]
    fn test_parse_regular_variant_with_important() {
        // Test that regular variants still work with important
        let input = "hover:!bg-blue-500";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse hover:!bg-blue-500");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set for hover:!bg-blue-500");
        assert_eq!(style.elements, vec!["bg", "blue", "500"]);
    }
    
    #[test]
    fn test_parse_container_query_important_with_negative() {
        // Test @lg:!-mt-4
        let input = "@lg:!-mt-4";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok(), "Failed to parse @lg:!-mt-4");
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert!(style.important, "Important flag should be set");
        assert!(style.negative, "Negative flag should be set");
        assert_eq!(style.elements, vec!["mt", "4"]);
    }
}