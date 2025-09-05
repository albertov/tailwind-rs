use super::*;

/// Tailwind size utility that sets both width and height
/// 
/// Examples:
/// - `size-0` → `width: 0; height: 0;`
/// - `size-10` → `width: 2.5rem; height: 2.5rem;`
/// - `size-full` → `width: 100%; height: 100%;`
/// - `size-[100px]` → `width: 100px; height: 100px;`
#[derive(Clone, Debug)]
pub struct TailwindSize {
    size: SizingUnit,
}

impl TailwindSize {
    #[inline]
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        // Reuse the existing SizingUnit parsing logic
        Ok(Self { 
            size: SizingUnit::parse(pattern, arbitrary)? 
        })
    }
}

impl Display for TailwindSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "size-{}", self.size)
    }
}

impl TailwindInstance for TailwindSize {
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        // Get the value for both width and height
        // For width, pass true; for height, pass false (controls vw vs vh for screen)
        let width_value = self.size.get_attribute(true);
        let height_value = self.size.get_attribute(false);
        
        css_attributes! {
            "width" => width_value,
            "height" => height_value
        }
    }
}
