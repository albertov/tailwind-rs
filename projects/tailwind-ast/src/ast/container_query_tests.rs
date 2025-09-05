#[cfg(test)]
mod container_query_tests {
    use super::super::*;
    
    #[test]
    fn test_parse_container_query_min_width() {
        let input = "@lg:";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "");
        assert!(variant.container);
        assert_eq!(variant.container_type, Some(ContainerQueryType::Min));
        assert_eq!(variant.names, vec!["lg"]);
        assert_eq!(variant.modifier, None);
        assert!(!variant.not);
        assert!(!variant.pseudo);
    }
    
    #[test]
    fn test_parse_container_query_max_width() {
        let input = "@max-lg:";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "");
        assert!(variant.container);
        assert_eq!(variant.container_type, Some(ContainerQueryType::Max));
        assert_eq!(variant.names, vec!["lg"]);
        assert_eq!(variant.modifier, None);
    }
    
    #[test]
    fn test_parse_container_query_min_explicit() {
        let input = "@min-lg:";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "");
        assert!(variant.container);
        assert_eq!(variant.container_type, Some(ContainerQueryType::Min));
        assert_eq!(variant.names, vec!["lg"]);
        assert_eq!(variant.modifier, None);
    }
    
    #[test]
    fn test_parse_container_query_arbitrary() {
        let input = "@[300px]:";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "");
        assert!(variant.container);
        assert_eq!(variant.container_type, Some(ContainerQueryType::Arbitrary));
        assert_eq!(variant.names, vec!["300px"]);
        assert_eq!(variant.modifier, None);
    }
    
    #[test]
    fn test_parse_container_query_arbitrary_rem() {
        let input = "@[20rem]:";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "");
        assert!(variant.container);
        assert_eq!(variant.container_type, Some(ContainerQueryType::Arbitrary));
        assert_eq!(variant.names, vec!["20rem"]);
        assert_eq!(variant.modifier, None);
    }
    
    #[test]
    fn test_parse_container_query_named() {
        let input = "@lg/sidebar:";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "");
        assert!(variant.container);
        assert_eq!(variant.container_type, Some(ContainerQueryType::Min));
        assert_eq!(variant.names, vec!["lg"]);
        assert_eq!(variant.modifier, Some("sidebar"));
    }
    
    #[test]
    fn test_parse_container_query_max_named() {
        let input = "@max-lg/header:";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "");
        assert!(variant.container);
        assert_eq!(variant.container_type, Some(ContainerQueryType::Max));
        assert_eq!(variant.names, vec!["lg"]);
        assert_eq!(variant.modifier, Some("header"));
    }
    
    #[test]
    fn test_parse_container_query_arbitrary_named() {
        let input = "@[500px]/main:";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "");
        assert!(variant.container);
        assert_eq!(variant.container_type, Some(ContainerQueryType::Arbitrary));
        assert_eq!(variant.names, vec!["500px"]);
        assert_eq!(variant.modifier, Some("main"));
    }
    
    #[test]
    fn test_parse_container_query_with_class() {
        let input = "@lg:flex";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "flex");
        assert!(variant.container);
        assert_eq!(variant.container_type, Some(ContainerQueryType::Min));
        assert_eq!(variant.names, vec!["lg"]);
    }
    
    #[test]
    fn test_parse_container_query_various_breakpoints() {
        let breakpoints = ["sm", "md", "lg", "xl", "2xl", "3xl"];
        
        for bp in &breakpoints {
            let input = format!("@{}:", bp);
            let result = ASTVariant::parse(&input);
            assert!(result.is_ok(), "Failed to parse @{}: variant", bp);
            
            let (_, variant) = result.unwrap();
            assert!(variant.container);
            assert_eq!(variant.container_type, Some(ContainerQueryType::Min));
            assert_eq!(variant.names, vec![*bp]);
        }
    }
    
    #[test]
    fn test_parse_regular_variant_not_container() {
        // Make sure regular variants still work
        let input = "hover:";
        let result = ASTVariant::parse(input);
        assert!(result.is_ok());
        
        let (rest, variant) = result.unwrap();
        assert_eq!(rest, "");
        assert!(!variant.container);
        assert_eq!(variant.container_type, None);
        assert_eq!(variant.names, vec!["hover"]);
    }
    
    #[test]
    fn test_parse_multiple_container_queries() {
        // Test parsing in a full style context
        let input = "@lg:flex";
        let styles = parse_tailwind(input);
        assert!(styles.is_ok());
        
        let styles = styles.unwrap();
        assert_eq!(styles.len(), 1);
        
        let style = &styles[0];
        assert_eq!(style.variants.len(), 1);
        assert!(style.variants[0].container);
        assert_eq!(style.elements, vec!["flex"]);
    }
}