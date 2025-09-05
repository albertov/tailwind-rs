use super::*;
use super::get_transform_property;

/// Sealed trait pattern to prevent external implementations
pub mod sealed {
    pub trait Sealed {}
}

/// A unified trait for all transform utilities (rotate, scale, translate, skew)
/// 
/// This trait provides a common interface for transform utilities while allowing
/// each utility to maintain its unique behavior through trait methods.
/// 
/// The trait handles:
/// - CSS variable generation for transform composition
/// - Transform property generation with all transformations composed
/// - Axis-specific variations (X, Y, or both)
pub trait TransformUtility: sealed::Sealed + Display + Clone + Debug {
    /// The type used to store the utility's value (e.g., UnitValue, NumericValue)
    type ValueType;
    
    /// Get the value stored in this utility
    fn get_value(&self) -> &Self::ValueType;
    
    /// Get the axis for this utility, if applicable
    /// - Some(AxisXY::X) for x-axis transforms (scale-x, translate-x, skew-x)
    /// - Some(AxisXY::Y) for y-axis transforms (scale-y, translate-y, skew-y)
    /// - Some(AxisXY::N) for both axes (scale, translate, skew without axis)
    /// - None for utilities without axis (rotate)
    fn get_axis(&self) -> Option<AxisXY>;
    
    /// Get the CSS variable prefix for this utility
    /// Examples: "--tw-rotate", "--tw-scale", "--tw-translate", "--tw-skew"
    fn css_var_prefix(&self) -> &'static str;
    
    /// Format the CSS value from the internal value representation
    /// This method converts the ValueType into the appropriate CSS string
    /// Examples:
    /// - Rotate: "45deg"
    /// - Scale: "1.5"
    /// - Translate: "2rem"
    /// - Skew: "12deg"
    fn format_css_value(&self, value: &Self::ValueType) -> String;
    
    /// Generate the complete CSS attributes for this utility
    /// 
    /// This default implementation:
    /// 1. Sets the appropriate CSS variables based on the axis
    /// 2. Includes the transform property with all transformations composed
    fn generate_css_attributes(&self) -> CssAttributes {
        let value = self.get_value();
        let css_value = self.format_css_value(value);
        let (transform_key, transform_value) = get_transform_property();
        
        match self.get_axis() {
            None => {
                // For utilities without axis (rotate)
                css_attributes! {
                    self.css_var_prefix() => css_value,
                    transform_key => transform_value,
                }
            }
            Some(AxisXY::X) => {
                // For x-axis specific utilities
                let var_name = format!("{}-x", self.css_var_prefix());
                css_attributes! {
                    var_name => css_value,
                    transform_key => transform_value,
                }
            }
            Some(AxisXY::Y) => {
                // For y-axis specific utilities
                let var_name = format!("{}-y", self.css_var_prefix());
                css_attributes! {
                    var_name => css_value,
                    transform_key => transform_value,
                }
            }
            Some(AxisXY::N) => {
                // For utilities that affect both axes
                let var_x = format!("{}-x", self.css_var_prefix());
                let var_y = format!("{}-y", self.css_var_prefix());
                css_attributes! {
                    var_x => css_value.clone(),
                    var_y => css_value,
                    transform_key => transform_value,
                }
            }
        }
    }
}

/// Blanket implementation of TailwindInstance for all TransformUtility implementors
/// 
/// This allows any type that implements TransformUtility to automatically
/// be a valid TailwindInstance without needing explicit implementations
impl<T> TailwindInstance for T 
where 
    T: TransformUtility 
{
    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
        self.generate_css_attributes()
    }
}