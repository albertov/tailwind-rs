use crate::systems::instruction::TailwindVariant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Breakpoint {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Custom(String),  // For user-defined breakpoints
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariantType {
    Responsive(Breakpoint),
    PseudoClass(PseudoClass),
    PseudoElement(PseudoElement),
    Dark,
    State(StateVariant),
    Unknown(String),  // Fallback for unrecognized variants
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoClass {
    Hover,
    Focus,
    Active,
    Visited,
    Disabled,
    Checked,
    Enabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoElement {
    Before,
    After,
    Marker,
    Selection,
    FirstLetter,
    FirstLine,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateVariant {
    First,
    Last,
    Even,
    Odd,
    FirstOfType,
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
    
    pub fn to_class_prefix(&self) -> String {
        self.names.join("-")
    }
}