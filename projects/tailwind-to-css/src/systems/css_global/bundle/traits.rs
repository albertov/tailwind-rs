use super::*;

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for CssBundle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.attribute.hash(state);
        // Hash the items in the IndexSet in order
        for item in &self.addition {
            item.hash(state);
        }
    }
}
