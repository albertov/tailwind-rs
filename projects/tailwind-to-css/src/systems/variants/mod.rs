//! Tailwind CSS variant type system.
//!
//! This module provides a type-safe representation of Tailwind CSS variants
//! including responsive breakpoints, pseudo-classes, pseudo-elements, and state variants.

use crate::systems::instruction::TailwindVariant;

/// Responsive breakpoint variants for media queries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Breakpoint {
    /// Small screens (640px)
    Sm,
    /// Medium screens (768px)
    Md,
    /// Large screens (1024px)
    Lg,
    /// Extra large screens (1280px)
    Xl,
    /// 2x extra large screens (1536px)
    Xxl,
    /// User-defined custom breakpoints
    Custom(String),
}

/// Type-safe representation of all Tailwind variant types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariantType {
    /// Responsive breakpoints (sm, md, lg, xl, 2xl)
    Responsive(Breakpoint),
    /// CSS pseudo-classes (hover, focus, active, etc.)
    PseudoClass(PseudoClass),
    /// CSS pseudo-elements (before, after, etc.)
    PseudoElement(PseudoElement),
    /// Dark mode variant
    Dark,
    /// Structural state variants (first, last, even, odd, etc.)
    State(StateVariant),
    /// Unknown variants (may be custom breakpoints or future features)
    Unknown(String),
}

/// CSS pseudo-class variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoClass {
    /// :hover pseudo-class
    Hover,
    /// :focus pseudo-class
    Focus,
    /// :active pseudo-class
    Active,
    /// :visited pseudo-class
    Visited,
    /// :disabled pseudo-class
    Disabled,
    /// :checked pseudo-class
    Checked,
    /// :enabled pseudo-class
    Enabled,
}

/// CSS pseudo-element variants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoElement {
    /// ::before pseudo-element
    Before,
    /// ::after pseudo-element
    After,
    /// ::marker pseudo-element
    Marker,
    /// ::selection pseudo-element
    Selection,
    /// ::first-letter pseudo-element
    FirstLetter,
    /// ::first-line pseudo-element
    FirstLine,
}

/// Structural state variants for element positioning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateVariant {
    /// :first-child selector
    First,
    /// :last-child selector
    Last,
    /// :nth-child(even) selector
    Even,
    /// :nth-child(odd) selector
    Odd,
    /// :first-of-type selector
    FirstOfType,
    /// :last-of-type selector
    LastOfType,
}

impl Breakpoint {
    /// Returns the breakpoint name for lookup in BreakPointSystem
    pub fn name(&self) -> &str {
        match self {
            Breakpoint::Sm => "sm",
            Breakpoint::Md => "md",
            Breakpoint::Lg => "lg",
            Breakpoint::Xl => "xl",
            Breakpoint::Xxl => "2xl",
            Breakpoint::Custom(name) => name.as_str(),
        }
    }
}

impl PseudoClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            PseudoClass::Hover => "hover",
            PseudoClass::Focus => "focus",
            PseudoClass::Active => "active",
            PseudoClass::Visited => "visited",
            PseudoClass::Disabled => "disabled",
            PseudoClass::Checked => "checked",
            PseudoClass::Enabled => "enabled",
        }
    }
}

impl PseudoElement {
    pub fn as_str(&self) -> &'static str {
        match self {
            PseudoElement::Before => "before",
            PseudoElement::After => "after",
            PseudoElement::Marker => "marker",
            PseudoElement::Selection => "selection",
            PseudoElement::FirstLetter => "first-letter",
            PseudoElement::FirstLine => "first-line",
        }
    }
}

impl StateVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            StateVariant::First => "first-child",
            StateVariant::Last => "last-child",
            StateVariant::Even => "nth-child(even)",
            StateVariant::Odd => "nth-child(odd)",
            StateVariant::FirstOfType => "first-of-type",
            StateVariant::LastOfType => "last-of-type",
        }
    }
}

impl TailwindVariant {
    /// Determines the variant type based on the variant name.
    /// 
    /// Maps string variant names to their corresponding type-safe enum representations.
    pub fn get_type(&self) -> VariantType {
        let name = self.names.join("-");
        match name.as_str() {
            // Standard breakpoints
            "sm" => VariantType::Responsive(Breakpoint::Sm),
            "md" => VariantType::Responsive(Breakpoint::Md),
            "lg" => VariantType::Responsive(Breakpoint::Lg),
            "xl" => VariantType::Responsive(Breakpoint::Xl),
            "2xl" => VariantType::Responsive(Breakpoint::Xxl),
            
            // Pseudo-classes
            "hover" => VariantType::PseudoClass(PseudoClass::Hover),
            "focus" => VariantType::PseudoClass(PseudoClass::Focus),
            "active" => VariantType::PseudoClass(PseudoClass::Active),
            "visited" => VariantType::PseudoClass(PseudoClass::Visited),
            "disabled" => VariantType::PseudoClass(PseudoClass::Disabled),
            "checked" => VariantType::PseudoClass(PseudoClass::Checked),
            "enabled" => VariantType::PseudoClass(PseudoClass::Enabled),
            
            // Pseudo-elements
            "before" => VariantType::PseudoElement(PseudoElement::Before),
            "after" => VariantType::PseudoElement(PseudoElement::After),
            "marker" => VariantType::PseudoElement(PseudoElement::Marker),
            "selection" => VariantType::PseudoElement(PseudoElement::Selection),
            
            // Dark mode
            "dark" => VariantType::Dark,
            
            // State variants
            "first" => VariantType::State(StateVariant::First),
            "last" => VariantType::State(StateVariant::Last),
            "even" => VariantType::State(StateVariant::Even),
            "odd" => VariantType::State(StateVariant::Odd),
            "first-of-type" => VariantType::State(StateVariant::FirstOfType),
            "last-of-type" => VariantType::State(StateVariant::LastOfType),
            
            _ => {
                // Unknown variant - could be a custom breakpoint or future Tailwind feature
                // During CSS generation, we'll check if it's a registered breakpoint
                VariantType::Unknown(name)
            }
        }
    }
    
    /// Converts the variant names to a class prefix string.
    /// 
    /// Joins multiple names with hyphens (e.g., ["first", "child"] becomes "first-child").
    pub fn to_class_prefix(&self) -> String {
        self.names.join("-")
    }
}