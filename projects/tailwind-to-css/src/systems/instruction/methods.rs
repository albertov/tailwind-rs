use super::*;

impl<'a> From<AstStyle<'a>> for TailwindInstruction {
    fn from(node: AstStyle<'a>) -> Self {
        Self {
            negative: Negative::from(node.negative),
            important: node.important,
            variants: node.variants.into_iter().map(|s| s.into()).collect(),
            elements: TailwindElements { inner: node.elements.into_iter().map(|s| s.to_string()).collect() },
            arbitrary: TailwindArbitrary::from(node.arbitrary.unwrap_or_default()),
            opacity: node.opacity.map(|s| s.to_string()),
        }
    }
}

impl<'a> From<ASTVariant<'a>> for TailwindVariant {
    fn from(node: ASTVariant<'a>) -> Self {
        Self { 
            not: node.not, 
            pseudo: node.pseudo, 
            names: node.names.into_iter().map(|s| s.to_string()).collect(),
            modifier: node.modifier.map(|s| s.to_string()),
            container: node.container,
            container_type: node.container_type,
            has: node.has,
            has_selector: node.has_selector.map(|s| s.to_string()),
            arbitrary_selector: node.arbitrary_selector.map(|s| s.to_string()),
        }
    }
}

impl TailwindInstruction {
    #[inline]
    pub fn view_elements(&self) -> Vec<&str> {
        self.elements.inner.iter().map(|s| s.as_str()).collect()
    }
    #[inline]
    pub fn view_arbitrary(&self) -> &TailwindArbitrary {
        &self.arbitrary
    }
    #[inline]
    pub fn view_variants(&self) -> &[TailwindVariant] {
        &self.variants
    }
    #[inline]
    pub fn view_opacity(&self) -> Option<&str> {
        self.opacity.as_deref()
    }
    #[inline]
    pub fn is_important(&self) -> bool {
        self.important
    }
    // TODO
    pub fn normalization(self) -> Self {
        self
    }
}
