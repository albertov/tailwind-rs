use std::collections::BTreeMap;

/// Container-specific breakpoint system separate from media queries.
/// 
/// Container breakpoints are optimized for component-level responsive design,
/// using different default values than viewport breakpoints.
#[derive(Clone, Debug, Default)]
pub struct ContainerBreakpointSystem {
    inner: BTreeMap<String, ContainerBreakpoint>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerBreakpoint {
    /// Value with unit (e.g., "16rem", "256px", "20ch")
    value: String,
}

impl ContainerBreakpointSystem {
    /// Creates the default container breakpoints optimized for components.
    /// 
    /// These values are based on Tailwind CSS v4's container query breakpoints,
    /// which are different from viewport breakpoints to better suit component-level design.
    pub fn builtin() -> Self {
        let mut inner = BTreeMap::new();
        
        // Container-optimized breakpoints (different from viewport)
        // Using Tailwind v3/v4 default values for container queries
        inner.insert("sm".to_string(), ContainerBreakpoint { value: "16rem".to_string() });  // 256px
        inner.insert("md".to_string(), ContainerBreakpoint { value: "28rem".to_string() });  // 448px
        inner.insert("lg".to_string(), ContainerBreakpoint { value: "48rem".to_string() });  // 768px
        inner.insert("xl".to_string(), ContainerBreakpoint { value: "64rem".to_string() });  // 1024px
        inner.insert("2xl".to_string(), ContainerBreakpoint { value: "80rem".to_string() }); // 1280px
        
        Self { inner }
    }
    
    /// Get a container breakpoint value.
    pub fn get(&self, name: &str) -> Result<&str, String> {
        self.inner
            .get(name)
            .map(|bp| bp.value.as_str())
            .ok_or_else(|| format!("Container breakpoint '{}' not found", name))
    }
    
    /// Get or generate CSS variable for a breakpoint.
    pub fn get_css_var(&self, name: &str) -> String {
        format!("var(--container-{})", name)
    }
    
    /// Generate CSS variables for all container breakpoints.
    /// 
    /// This generates a :root rule with all container breakpoint CSS variables,
    /// allowing them to be referenced in @container rules.
    pub fn generate_css_variables(&self) -> String {
        let mut css = String::from(":root {\n");
        
        for (name, breakpoint) in &self.inner {
            css.push_str(&format!("  --container-{}: {};\n", name, breakpoint.value));
        }
        
        css.push_str("}\n");
        css
    }
    
    /// Register a custom container breakpoint.
    pub fn register(&mut self, name: String, value: String) -> Option<ContainerBreakpoint> {
        self.inner.insert(name, ContainerBreakpoint { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builtin_breakpoints() {
        let system = ContainerBreakpointSystem::builtin();
        
        // Test standard breakpoints
        assert_eq!(system.get("sm").unwrap(), "16rem");
        assert_eq!(system.get("md").unwrap(), "28rem");
        assert_eq!(system.get("lg").unwrap(), "48rem");
        assert_eq!(system.get("xl").unwrap(), "64rem");
        assert_eq!(system.get("2xl").unwrap(), "80rem");
    }
    
    #[test]
    fn test_get_nonexistent_breakpoint() {
        let system = ContainerBreakpointSystem::builtin();
        let result = system.get("3xl");
        assert!(result.is_err());
        
        if let Err(msg) = result {
            assert!(msg.contains("3xl"));
            assert!(msg.contains("not found"));
        }
    }
    
    #[test]
    fn test_css_variable_generation() {
        let system = ContainerBreakpointSystem::builtin();
        
        assert_eq!(system.get_css_var("lg"), "var(--container-lg)");
        assert_eq!(system.get_css_var("2xl"), "var(--container-2xl)");
    }
    
    #[test]
    fn test_generate_all_css_variables() {
        let mut system = ContainerBreakpointSystem::default();
        system.register("test".to_string(), "10rem".to_string());
        
        let css = system.generate_css_variables();
        assert!(css.contains(":root {"));
        assert!(css.contains("--container-test: 10rem;"));
        assert!(css.contains("}"));
    }
    
    #[test]
    fn test_register_custom_breakpoint() {
        let mut system = ContainerBreakpointSystem::builtin();
        
        system.register("3xl".to_string(), "96rem".to_string());
        assert_eq!(system.get("3xl").unwrap(), "96rem");
    }
}