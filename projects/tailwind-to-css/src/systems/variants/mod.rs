//! Tailwind CSS variant type system.
//!
//! This module provides a type-safe representation of Tailwind CSS variants
//! including responsive breakpoints, pseudo-classes, pseudo-elements, and state variants.

use crate::systems::instruction::TailwindVariant;

/// Type of container query
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContainerQueryType {
    /// Minimum width query (@lg: or @min-lg:)
    MinWidth,
    /// Maximum width query (@max-lg:)
    MaxWidth,
    /// Arbitrary value query (@[123px]:)
    Arbitrary,
}

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
    /// Group variants (group-hover, group-focus, group-[arbitrary], etc.)
    Group {
        state: String,
        modifier: Option<String>,
        /// Optional arbitrary selector for group-[selector]
        arbitrary_selector: Option<String>,
    },
    /// Peer variants (peer-checked, peer-hover, peer-[arbitrary], etc.)
    Peer {
        state: String,
        modifier: Option<String>,
        /// Optional arbitrary selector for peer-[selector]
        arbitrary_selector: Option<String>,
    },
    /// Has variants for descendant state styling (has-checked:, has-[>img]:)
    Has {
        /// The selector or state to check for (e.g., "checked", ">img")
        selector: String,
        /// True if this is an arbitrary selector from brackets
        arbitrary: bool,
        /// True if this should be negated (has-not-checked)
        negated: bool,
    },
    /// Group-has variants for descendant state within groups
    GroupHas {
        /// The selector or state to check for
        selector: String,
        /// Optional group modifier (e.g., "sidebar")
        modifier: Option<String>,
        /// True if this is an arbitrary selector
        arbitrary: bool,
        /// True if this should be negated
        negated: bool,
    },
    /// Peer-has variants for descendant state within peers
    PeerHas {
        /// The selector or state to check for
        selector: String,
        /// Optional peer modifier (e.g., "input")
        modifier: Option<String>,
        /// True if this is an arbitrary selector
        arbitrary: bool,
        /// True if this should be negated
        negated: bool,
    },
    /// Container query variants (@lg:, @max-lg:, @[123px]:, @lg/sidebar:)
    ContainerQuery {
        /// Type of query (min-width, max-width, arbitrary)
        query_type: ContainerQueryType,
        /// Breakpoint name (e.g., "lg", "md") or arbitrary value
        breakpoint: String,
        /// Optional container name (e.g., "sidebar")
        container_name: Option<String>,
    },
    /// Arbitrary responsive breakpoints (min-[400px]:, max-[600px]:)
    ArbitraryResponsive {
        /// True for max-width, false for min-width
        is_max: bool,
        /// The breakpoint value (e.g., "400px", "50rem")
        value: String,
    },
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
        // Check if this is a container query variant
        if self.container {
            let breakpoint = self.names.first()
                .map(|s| s.to_string())
                .unwrap_or_default();
            
            // Map AST ContainerQueryType to variant ContainerQueryType
            let query_type = match self.container_type.as_ref() {
                Some(tailwind_ast::ContainerQueryType::Min) => ContainerQueryType::MinWidth,
                Some(tailwind_ast::ContainerQueryType::Max) => ContainerQueryType::MaxWidth,
                Some(tailwind_ast::ContainerQueryType::Arbitrary) => ContainerQueryType::Arbitrary,
                None => ContainerQueryType::MinWidth, // Default to min-width
            };
            
            return VariantType::ContainerQuery {
                query_type,
                breakpoint,
                container_name: self.modifier.clone(),
            };
        }
        
        let name = self.names.join("-");
        
        // Check for has variants first
        if self.has {
            // For arbitrary has selectors, names might be empty
            if self.names.is_empty() && self.has_selector.is_some() {
                let selector = self.has_selector.as_ref().unwrap().clone();
                return VariantType::Has {
                    selector,
                    arbitrary: true,
                    negated: self.not,
                };
            }
            
            // Check if this is a group-has or peer-has variant
            // When has_selector is set, the parser doesn't include "has" in names
            if self.has_selector.is_some() && !self.names.is_empty() {
                let prefix = &self.names[0];
                let selector = self.has_selector.as_ref().unwrap().clone();
                
                match prefix.as_str() {
                    "group" => {
                        return VariantType::GroupHas {
                            selector,
                            modifier: self.modifier.clone(),
                            arbitrary: true,
                            negated: self.not,
                        };
                    }
                    "peer" => {
                        return VariantType::PeerHas {
                            selector,
                            modifier: self.modifier.clone(),
                            arbitrary: true,
                            negated: self.not,
                        };
                    }
                    _ => {
                        // Unknown prefix, treat as basic has
                        return VariantType::Has {
                            selector,
                            arbitrary: true,
                            negated: self.not,
                        };
                    }
                }
            }
            
            // Find the index of "has" in names (for non-arbitrary selectors)
            let has_idx = self.names.iter().position(|n| n == "has");
            
            if let Some(idx) = has_idx {
                // Check what comes before "has"
                if idx > 0 {
                    let prefix = &self.names[0];
                    
                    // Determine selector: arbitrary or from names after "has"
                    let selector = if let Some(arb) = &self.has_selector {
                        arb.clone()
                    } else if idx + 1 < self.names.len() {
                        // Join the parts after "has" as the selector
                        self.names[idx + 1..].join("-")
                    } else {
                        String::new()
                    };
                    
                    match prefix.as_str() {
                        "group" => {
                            return VariantType::GroupHas {
                                selector,
                                modifier: self.modifier.clone(),
                                arbitrary: self.has_selector.is_some(),
                                negated: self.not,
                            };
                        }
                        "peer" => {
                            return VariantType::PeerHas {
                                selector,
                                modifier: self.modifier.clone(),
                                arbitrary: self.has_selector.is_some(),
                                negated: self.not,
                            };
                        }
                        _ => {
                            // Unknown prefix, treat as basic has
                            return VariantType::Has {
                                selector,
                                arbitrary: self.has_selector.is_some(),
                                negated: self.not,
                            };
                        }
                    }
                } else {
                    // "has" is at the beginning
                    let selector = if let Some(arb) = &self.has_selector {
                        arb.clone()
                    } else if idx + 1 < self.names.len() {
                        // Join the parts after "has" as the selector
                        self.names[idx + 1..].join("-")
                    } else {
                        String::new()
                    };
                    
                    return VariantType::Has {
                        selector,
                        arbitrary: self.has_selector.is_some(),
                        negated: self.not,
                    };
                }
            }
        }
        
        // Check for group/peer with arbitrary selectors first
        if self.names.len() == 1 {
            if self.names[0] == "group" && self.arbitrary_selector.is_some() {
                return VariantType::Group {
                    state: String::new(), // No specific state, using arbitrary selector
                    modifier: self.modifier.clone(),
                    arbitrary_selector: self.arbitrary_selector.clone(),
                };
            }
            if self.names[0] == "peer" && self.arbitrary_selector.is_some() {
                return VariantType::Peer {
                    state: String::new(), // No specific state, using arbitrary selector
                    modifier: self.modifier.clone(),
                    arbitrary_selector: self.arbitrary_selector.clone(),
                };
            }
        }
        
        // Check for regular group variants (e.g., group-hover, group-focus)
        if name.starts_with("group-") {
            let state = name.strip_prefix("group-").unwrap().to_string();
            return VariantType::Group {
                state,
                modifier: self.modifier.clone(),
                arbitrary_selector: None,
            };
        }
        
        // Check for regular peer variants (e.g., peer-checked, peer-hover)
        if name.starts_with("peer-") {
            let state = name.strip_prefix("peer-").unwrap().to_string();
            return VariantType::Peer {
                state,
                modifier: self.modifier.clone(),
                arbitrary_selector: None,
            };
        }
        
        // Check for arbitrary responsive breakpoints (min-[...] or max-[...])
        // These are parsed with names = ["min"] or ["max"] and arbitrary_selector = value
        if (name == "min" || name == "max") && self.arbitrary_selector.is_some() {
            return VariantType::ArbitraryResponsive {
                is_max: name == "max",
                value: self.arbitrary_selector.as_ref().unwrap().clone(),
            };
        }
        
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
}