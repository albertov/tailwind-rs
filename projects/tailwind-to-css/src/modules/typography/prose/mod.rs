use crate::modules::*;

/// Basic implementation of prose utility for typography
#[derive(Copy, Clone, Debug, Default)]
pub struct TailwindProse {}

impl Display for TailwindProse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "prose")
    }
}

impl TailwindInstance for TailwindProse {
    fn inlineable(&self) -> bool {
        false
    }
    
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // Basic prose styles - in a full implementation this would include
        // comprehensive typography styles
        css_attributes! {
            "color" => "rgb(55 65 81)",
            "max-width" => "65ch"
        }
    }
}

impl TailwindProse {
    pub fn default() -> Self {
        Self {}
    }
    
    pub fn boxed(self) -> Box<dyn TailwindInstance> {
        Box::new(self)
    }
}