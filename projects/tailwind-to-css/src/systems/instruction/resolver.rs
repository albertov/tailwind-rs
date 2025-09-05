use super::*;

// noinspection SpellCheckingInspection
impl TailwindInstruction {
    #[inline(never)]
    pub fn get_instance(&self) -> Result<Box<dyn TailwindInstance>> {
        let element = self.view_elements();
        let pattern = element.as_slice();
        let arbitrary = self.view_arbitrary();
        let neg = self.negative;
        let instance = match pattern {
            // Layout System
            ["aspect", rest @ ..] => TailwindAspect::parse(rest, arbitrary)?.boxed(),
            ["container"] => TailwindContainer::default().boxed(),
            // Container Query Setup utilities
            ["@container"] => {
                // Extract container name from opacity field (which captures the /name part)
                let name = self.view_opacity().map(|s| s.to_string());
                ContainerQuerySetup::inline_size(name).boxed()
            },
            ["@container", "normal"] => {
                let name = self.view_opacity().map(|s| s.to_string());
                ContainerQuerySetup::normal(name).boxed()
            },
            ["@container", "size"] => {
                let name = self.view_opacity().map(|s| s.to_string());
                ContainerQuerySetup::size(name).boxed()
            },
            ["columns", rest @ ..] => TailwindColumns::parse(rest, arbitrary)?.boxed(),
            ["break", rest @ ..] => TailwindBreak::parse(rest, arbitrary)?,
            ["box", rest @ ..] => Self::box_adaptor(rest, arbitrary)?,
            // begin https://tailwindcss.com/docs/display
            // skip [flex, table]
            ["block"] => TailwindDisplay::from("block").boxed(),
            ["inline", "block"] => TailwindDisplay::from("inline-block").boxed(),
            ["inline"] => TailwindDisplay::from("inline").boxed(),
            ["inline", "flex"] => TailwindDisplay::from("inline-flex").boxed(),
            ["inline", "table"] => TailwindDisplay::from("inline-table").boxed(),
            ["flow", "root"] => TailwindDisplay::from("flow-root").boxed(),
            ["grid"] => TailwindDisplay::from("grid").boxed(),
            ["inline", "grid"] => TailwindDisplay::from("inline-grid").boxed(),
            ["contents"] => TailwindDisplay::from("contents").boxed(),
            ["list", "item"] => TailwindDisplay::from("inline-grid").boxed(),
            ["hidden"] => TailwindDisplay::from("hidden").boxed(),
            // https://tailwindcss.com/docs/float
            ["float", rest @ ..] => TailwindFloat::parse(rest, arbitrary)?.boxed(),
            ["clear", rest @ ..] => TailwindClear::parse(rest, arbitrary)?.boxed(),
            ["isolate"] => TailwindIsolation::from("isolate").boxed(),
            ["isolation", rest @ ..] => TailwindIsolation::parse(rest, arbitrary)?.boxed(),
            ["object", rest @ ..] => object_adaptor(rest, arbitrary)?,
            ["overflow", rest @ ..] => TailwindOverflow::parse(rest, arbitrary)?.boxed(),
            ["overscroll", rest @ ..] => TailwindOverscroll::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/position#header
            [s @ ("static" | "fixed" | "absolute" | "relative" | "sticky")] => TailwindPosition::from(*s).boxed(),
            ["position", rest @ ..] => TailwindPosition::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/top-right-bottom-left
            // Special case: inset-ring utilities must be checked before regular inset positioning
            ["inset", "ring", rest @ ..] => Self::inset_ring_adaptor(rest, arbitrary)?,
            ["inset", rest @ ..] => TailwindInset::parse(rest, arbitrary, neg)?.boxed(),
            ["top", rest @ ..] => TailwindTop::parse(rest, arbitrary, neg)?.boxed(),
            ["right", rest @ ..] => TailwindRight::parse(rest, arbitrary, neg)?.boxed(),
            ["bottom", rest @ ..] => TailwindBottom::parse(rest, arbitrary, neg)?.boxed(),
            ["left", rest @ ..] => TailwindLeft::parse(rest, arbitrary, neg)?.boxed(),
            ["start", rest @ ..] => TailwindStart::parse(rest, arbitrary, neg)?.boxed(),
            ["end", rest @ ..] => TailwindEnd::parse(rest, arbitrary, neg)?.boxed(),
            // https://tailwindcss.com/docs/visibility
            ["invisible"] => TailwindVisibility::from("hidden").boxed(),
            ["visible" | "visibility", rest @ ..] => TailwindVisibility::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/z-index
            ["z", rest @ ..] => TailwindZIndex::parse(rest, arbitrary, neg)?.boxed(),
            // Flexbox & Grid
            ["basis", rest @ ..] => TailwindBasis::parse(rest, arbitrary)?.boxed(),
            ["flex", rest @ ..] => TailwindFlex::adapt(rest, arbitrary)?,
            ["grow", rest @ ..] => TailWindGrow::parse(rest, arbitrary)?.boxed(),
            ["shrink", rest @ ..] => TailWindShrink::parse(rest, arbitrary)?.boxed(),
            ["order", rest @ ..] => TailWindOrder::parse(rest, arbitrary, neg)?.boxed(),
            ["grid", rest @ ..] => TailwindGrid::adapt(rest, arbitrary)?,
            // https://tailwindcss.com/docs/grid-column
            ["col", rest @ ..] => TailwindColumn::parse(rest, arbitrary)?.boxed(),
            ["row", rest @ ..] => TailwindRow::parse(rest, arbitrary)?.boxed(),
            ["auto", rest @ ..] => TailwindGridAuto::parse(rest, arbitrary)?.boxed(),
            ["gap", rest @ ..] => TailwindGap::parse(rest, arbitrary)?.boxed(),
            ["justify", rest @ ..] => justify_adaptor(rest, arbitrary)?,
            ["content", rest @ ..] => TailwindPseudoContent::adapt(rest, arbitrary)?,
            ["items", rest @ ..] => TailwindItems::parse(rest, arbitrary)?.boxed(),
            ["self", rest @ ..] => TailwindSelf::parse(rest, arbitrary)?.boxed(),
            ["place", rest @ ..] => TailwindPlace::adapt(rest, arbitrary)?,
            // justify catched
            // Spacing System
            ["p" | "pl" | "pr" | "pb" | "pt" | "px" | "py" | "ps" | "pe", ..] => TailwindPadding::parse(pattern, arbitrary, neg)?.boxed(),
            ["m" | "ml" | "mr" | "mb" | "mt" | "mx" | "my" | "ms" | "me", ..] => TailwindMargin::parse(pattern, arbitrary, neg)?.boxed(),
            ["space", rest @ ..] => TailwindSpace::parse(rest, arbitrary, neg)?,
            // Sizing System
            ["w", rest @ ..] => TailwindSizing::parse_width(rest, arbitrary)?.boxed(),
            ["min", "w", rest @ ..] => TailwindSizing::parse_width_min(rest, arbitrary)?.boxed(),
            ["max", "w", rest @ ..] => TailwindSizing::parse_width_max(rest, arbitrary)?.boxed(),
            ["h", rest @ ..] => TailwindSizing::parse_height(rest, arbitrary)?.boxed(),
            ["min", "h", rest @ ..] => TailwindSizing::parse_height_min(rest, arbitrary)?.boxed(),
            ["max", "h", rest @ ..] => TailwindSizing::parse_height_max(rest, arbitrary)?.boxed(),
            ["size", rest @ ..] => TailwindSize::parse(rest, arbitrary)?.boxed(),
            // Typography System
            ["font", rest @ ..] => font_adaptor(rest, arbitrary)?,
            ["text", rest @ ..] => text_adaptor(rest, arbitrary, self.view_opacity())?,
            // begin https://tailwindcss.com/docs/font-variant-numeric
            ["antialiased"] => TailwindFontSmoothing::from("todo").boxed(),
            ["subpixel", "antialiased"] => TailwindFontSmoothing::from("todo").boxed(),
            ["italic"] => TailwindFontStyle::from("italic").boxed(),
            ["not", "italic"] => TailwindFontStyle::from("normal").boxed(),
            // https://tailwindcss.com/docs/font-variant-numeric
            ["normal", "nums"] => TailwindFontVariantNumeric::from("normal").boxed(),
            ["ordinal"] => TailwindFontVariantNumeric::from("ordinal").boxed(),
            ["slashed", "zero"] => TailwindFontVariantNumeric::from("slashed-zero").boxed(),
            ["lining", "nums"] => TailwindFontVariantNumeric::from("lining-nums").boxed(),
            ["oldstyle", "nums"] => TailwindFontVariantNumeric::from("oldstyle-nums").boxed(),
            ["proportional", "nums"] => TailwindFontVariantNumeric::from("proportional-nums").boxed(),
            ["tabular", "nums"] => TailwindFontVariantNumeric::from("tabular-nums").boxed(),
            ["diagonal", "fractions"] => TailwindFontVariantNumeric::from("diagonal-fractions").boxed(),
            ["stacked", "fractions"] => TailwindFontVariantNumeric::from("stacked-fractions").boxed(),
            // https://tailwindcss.com/docs/letter-spacing
            ["tracking", rest @ ..] => TailwindTracking::parse(rest, arbitrary)?.boxed(),
            ["leading", rest @ ..] => TailwindLeading::parse(rest, arbitrary)?.boxed(),
            ["list", rest @ ..] => list_adaptor(rest, arbitrary)?,
            // https://tailwindcss.com/docs/text-decoration
            ["underline"] => TailwindDecorationLine::from("underline").boxed(),
            ["overline"] => TailwindDecorationLine::from("overline").boxed(),
            ["line", "through"] => TailwindDecorationLine::from("line-through").boxed(),
            ["no", "underline"] => TailwindDecorationLine::from("none").boxed(),
            // https://tailwindcss.com/docs/text-decoration-color
            ["decoration", rest @ ..] => TailwindDecoration::adapt(rest, arbitrary)?,
            ["underline", "offset", rest @ ..] => TailwindUnderlineOffset::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/text-transform
            ["uppercase"] => TailwindTextTransform::from("uppercase").boxed(),
            ["lowercase"] => TailwindTextTransform::from("lowercase").boxed(),
            ["capitalize"] => TailwindTextTransform::from("capitalize").boxed(),
            ["normal", "case"] => TailwindTextTransform::from("none").boxed(),
            // https://tailwindcss.com/docs/text-overflow
            ["truncate"] => TailwindTextOverflow::Truncate.boxed(),
            ["indent", rest @ ..] => TailwindIndent::parse(rest, arbitrary)?.boxed(),
            ["align", rest @ ..] => TailwindAlign::parse(rest, arbitrary)?.boxed(),
            ["whitespace", rest @ ..] => TailwindWhiteSpace::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/line-clamp
            ["line", "clamp", rest @ ..] => TailwindLineClamp::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/hyphens
            ["hyphens", rest @ ..] => TailwindHyphens::parse(rest, arbitrary)?.boxed(),
            // break catched
            // content catched
            // Typography System Extension
            ["prose"] => TailwindProse::default().boxed(),
            // Backgrounds System
            ["bg", rest @ ..] => Self::bg_adaptor(rest, arbitrary, self.view_opacity())?,
            ["from", rest @ ..] => TailwindFrom::parse(rest, arbitrary)?.boxed(),
            ["via", rest @ ..] => TailwindVia::parse(rest, arbitrary)?.boxed(),
            ["to", rest @ ..] => TailwindTo::parse(rest, arbitrary)?.boxed(),
            // Borders System
            ["rounded", rest @ ..] => TailwindRounded::parse(rest, arbitrary)?.boxed(),
            ["border", rest @ ..] => Self::border_adaptor(rest, arbitrary, self.view_opacity())?,
            ["divide", rest @ ..] => TailwindDivide::adapt(rest, arbitrary)?,
            ["outline", rest @ ..] => outline_adaptor(rest, arbitrary)?,
            ["ring", rest @ ..] => TailwindRing::adapt_with_opacity(rest, arbitrary, self.view_opacity())?,
            // Effects System
            ["shadow", rest @ ..] => Self::shadow_adaptor(rest, arbitrary, self.view_opacity())?,
            ["opacity", rest @ ..] => TailwindOpacity::parse(rest, arbitrary, false)?.boxed(),
            ["mix", "blend", rest @ ..] => TailwindBlend::parse(rest, arbitrary)?.boxed(),
            // Filters System
            ["blur", rest @ ..] => TailwindBlur::parse(rest, arbitrary, false)?.boxed(),
            ["brightness", rest @ ..] => TailwindBrightness::parse(rest, arbitrary, false)?.boxed(),
            ["contrast", rest @ ..] => TailwindContrast::parse(rest, arbitrary, false)?.boxed(),
            ["drop", "shadow", rest @ ..] => TailwindShadow::parse(rest, arbitrary, true)?.boxed(),
            ["grayscale", rest @ ..] => TailwindGrayscale::parse(rest, arbitrary, false)?.boxed(),
            ["hue", "rotate", rest @ ..] => TailwindHueRotate::parse(rest, arbitrary, false, neg)?.boxed(),
            ["invert", rest @ ..] => TailwindInvert::parse(rest, arbitrary, false)?.boxed(),
            ["saturate", rest @ ..] => TailwindSaturate::parse(rest, arbitrary, false)?.boxed(),
            ["sepia", rest @ ..] => TailwindSepia::parse(rest, arbitrary, false)?.boxed(),
            ["backdrop", rest @ ..] => Self::backdrop_adaptor(rest, arbitrary, neg)?,
            // Tables System
            ["caption", rest @ ..] => Self::caption_adaptor(rest, arbitrary)?,
            ["table", rest @ ..] => Self::table_adaptor(rest, arbitrary)?,
            // Transitions System
            ["transition", rest @ ..] => TailwindTransition::parse(rest, arbitrary)?.boxed(),
            ["duration", rest @ ..] => TailwindDuration::parse(rest, arbitrary)?.boxed(),
            ["ease", rest @ ..] => TailwindEase::parse(rest, arbitrary)?.boxed(),
            ["delay", rest @ ..] => TailwindDelay::parse(rest, arbitrary)?.boxed(),
            ["animate", rest @ ..] => TailwindAnimate::parse(rest, arbitrary)?.boxed(),
            // Transforms System
            ["scale", rest @ ..] => TailwindScale::parse(rest, arbitrary, neg)?.boxed(),
            ["rotate", rest @ ..] => TailwindRotate::parse(rest, arbitrary, neg)?.boxed(),
            ["translate", rest @ ..] => TailwindTranslate::parse(rest, arbitrary, neg)?.boxed(),
            ["skew", rest @ ..] => TailwindSkew::parse(rest, arbitrary, neg)?.boxed(),
            ["origin", rest @ ..] => TailwindOrigin::parse(rest, arbitrary)?.boxed(),
            ["transform", rest @ ..] => Self::transform_adaptor(rest, arbitrary)?,
            ["perspective", "origin", rest @ ..] => TailwindPerspectiveOrigin::parse(rest, arbitrary)?.boxed(),
            ["perspective", rest @ ..] => TailwindPerspective::parse(rest, arbitrary)?.boxed(),
            // Interactivity System
            ["accent", rest @ ..] => TailwindAccentColor::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/appearance
            ["appearance", rest @ ..] => TailwindAppearance::parse(rest, arbitrary)?.boxed(),
            ["cursor", rest @ ..] => TailwindCursor::parse(rest, arbitrary)?.boxed(),
            ["caret", rest @ ..] => TailwindCaretColor::parse(rest, arbitrary)?.boxed(),
            ["pointer", "events", rest @ ..] => TailwindPointerEvents::parse(rest, arbitrary)?.boxed(),
            ["resize", rest @ ..] => TailwindResize::parse(rest, arbitrary)?.boxed(),
            ["scroll", rest @ ..] => scroll_adaptor(rest, arbitrary, neg)?,
            ["snap", rest @ ..] => snap_adaptor(rest, arbitrary)?,
            ["touch", rest @ ..] => TailwindTorch::parse(rest, arbitrary)?.boxed(),
            ["select", rest @ ..] => TailwindSelect::parse(rest, arbitrary)?.boxed(),
            ["will", "change", rest @ ..] => TailwindWillChange::parse(rest, arbitrary)?.boxed(),
            // SVG System
            ["fill", "opacity", rest @ ..] => TailwindFillOpacity::parse(rest, arbitrary)?.boxed(),
            ["fill", rest @ ..] => TailwindFillColor::parse_with_opacity(rest, arbitrary, self.view_opacity())?.boxed(),
            ["stroke", rest @ ..] => TailwindStroke::parse(rest, arbitrary)?,
            // Accessibility System
            ["sr", "only"] => TailwindScreenReader::new(true).boxed(),
            ["not", "sr", "only"] => TailwindScreenReader::new(false).boxed(),
            // Form System Extension
            ["form", "input"] => TailwindFormInput::new().boxed(),
            ["form", "textarea"] => TailwindFormTextarea::new().boxed(),
            ["form", "select"] => TailwindFormSelect::new().boxed(),
            ["form", "checkbox"] => TailwindFormCheckbox::new().boxed(),
            ["form", "radio"] => TailwindFormRadio::new().boxed(),
            // Marker Utilities (group and peer)
            ["group"] => TailwindGroupMarker.boxed(),
            ["peer"] => TailwindPeerMarker.boxed(),
            _ => return syntax_error!("Unknown instructions: {} + {}", element.join("-"), arbitrary.get_class()),
        };
        Ok(instance)
    }
    #[inline]
    fn bg_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary, opacity: Option<&str>) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/background-attachment
            [s @ ("fixed" | "local" | "scroll")] => TailwindBackgroundAttachment::from(*s).boxed(),
            ["attach", rest @ ..] => TailwindBackgroundAttachment::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-clip
            ["clip", rest @ ..] => TailwindBackgroundClip::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-origin
            ["origin", rest @ ..] => TailwindBackgroundOrigin::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-position
            [s @ ("bottom" | "center" | "left" | "right" | "top")] => TailwindBackgroundPosition::parse(&[s], arbitrary)?.boxed(),
            ["left", s @ ("bottom" | "top")] => TailwindBackgroundPosition::parse(&["left", s], arbitrary)?.boxed(),
            ["right", s @ ("bottom" | "top")] => TailwindBackgroundPosition::parse(&["right", s], arbitrary)?.boxed(),
            // Arbitrary background position (e.g., bg-[25%_75%])
            // But exclude color functions
            [] if arbitrary.get_properties().contains('%') && arbitrary.get_properties().contains(' ') => {
                let value = arbitrary.as_str();
                // Check if it's a color function (don't treat as position)
                if value.starts_with("oklch(") || value.starts_with("lch(") || 
                   value.starts_with("lab(") || value.starts_with("oklab(") ||
                   value.starts_with("hwb(") || value.starts_with("color(") {
                    // This is a color, not a position - fall through to color parsing
                    TailwindBackgroundColor::parse_with_opacity(pattern, arbitrary, opacity)?.boxed()
                } else {
                    // This is a position
                    TailwindBackgroundPosition::parse(&[], arbitrary)?.boxed()
                }
            },
            // https://tailwindcss.com/docs/background-repeat
            ["no", "repeat"] => TailwindBackgroundRepeat::from("no-repeat").boxed(),
            ["repeat", rest @ ..] => TailwindBackgroundRepeat::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-size
            [s @ ("auto" | "cover" | "contain")] => TailwindBackgroundSize::from(*s).boxed(),
            ["size", rest @ ..] => TailwindBackgroundSize::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-blend-mode
            ["blend", rest @ ..] => TailwindBackgroundBlend::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/background-image
            ["none"] => {
                // Create a simple instance for bg-none
                struct BgNone;
                impl TailwindInstance for BgNone {
                    fn attributes(&self, _: &TailwindBuilder) -> CssAttributes {
                        css_attributes! {
                            "background-image" => "none"
                        }
                    }
                }
                impl Display for BgNone {
                    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                        write!(f, "bg-none")
                    }
                }
                BgNone.boxed()
            },
            ["gradient", "to", rest @ ..] => TailwindBackgroundGradient::parse(rest, arbitrary)?.boxed(),
            _ => TailwindBackgroundColor::parse_with_opacity(pattern, arbitrary, opacity)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn border_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary, opacity: Option<&str>) -> Result<Box<dyn TailwindInstance>> {
        let color = |color| TailwindBorderColor::from(color).boxed();
        let out = match pattern {
            // https://tailwindcss.com/docs/border-style
            [s @ ("solid" | "dashed" | "dotted" | "double" | "hidden" | "none")] => TailwindBorderStyle::from(*s).boxed(),
            // https://tailwindcss.com/docs/border-collapse
            ["separate"] => TailwindBorderCollapse::from("separate").boxed(),
            ["collapse"] if arbitrary.is_none() => TailwindBorderCollapse::from("collapse").boxed(),
            ["collapse", rest @ ..] => TailwindBorderCollapse::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/border-spacing
            ["spacing", rest @ ..] => TailwindBorderSpacing::parse(rest, arbitrary)?.boxed(),
            // https://tailwindcss.com/docs/border-width
            [] => {
                // Check if arbitrary value is a color
                if arbitrary.is_some() {
                    let value = arbitrary.as_str();
                    // Check if it's a color value
                    if value.starts_with('#') || value.starts_with("rgb") || value.starts_with("hsl") ||
                       value.starts_with("oklch(") || value.starts_with("lch(") || 
                       value.starts_with("lab(") || value.starts_with("oklab(") ||
                       value.starts_with("hwb(") || value.starts_with("color(") ||
                       value.starts_with("var(--") {
                        TailwindBorderColor::parse_with_opacity(pattern, arbitrary, opacity)?.boxed()
                    } else {
                        // Assume it's a width value (e.g. border-[3px])
                        TailwindBorderWidth::parse(pattern, arbitrary)?.boxed()
                    }
                } else {
                    TailwindBorderWidth::parse(pattern, arbitrary)?.boxed()
                }
            },
            ["0" | "2" | "4" | "8", ..] if arbitrary.is_none() => TailwindBorderWidth::parse(pattern, arbitrary)?.boxed(), // e.g. border-4
            // Handle sided borders - check if it's width or color
            ["x" | "y" | "t" | "r" | "b" | "l" | "s" | "e", rest @ ..] => {
                // If there's an arbitrary value, check if it looks like a width (contains px, rem, %, etc) or a color
                if arbitrary.is_some() {
                    let arb_value = arbitrary.get_properties();
                    // Check if it's a color value (starts with #, rgb, rgba, hsl, hsla)
                    if arb_value.starts_with('#') || 
                       arb_value.starts_with("rgb") || 
                       arb_value.starts_with("hsl") ||
                       arb_value.starts_with("var(--") {
                        TailwindBorderColor::parse_with_opacity(pattern, arbitrary, opacity)?.boxed()
                    } else {
                        // Assume it's a width value
                        TailwindBorderWidth::parse(pattern, arbitrary)?.boxed()
                    }
                } else if rest.is_empty() || matches!(rest.first(), Some(&"0" | &"1" | &"2" | &"4" | &"8")) {
                    // Check if the rest is a width value (empty, or starts with a width number)
                    TailwindBorderWidth::parse(pattern, arbitrary)?.boxed()
                } else {
                    // Otherwise it's a color
                    TailwindBorderColor::parse_with_opacity(pattern, arbitrary, opacity)?.boxed()
                }
            },
            // https://tailwindcss.com/docs/border-color
            ["black"] => color(TailwindColor::Black),
            ["white"] => color(TailwindColor::White),
            _ => TailwindBorderColor::parse_with_opacity(pattern, arbitrary, opacity)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn shadow_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary, opacity: Option<&str>) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/box-shadow
            ["black" | "white" | "current" | "transparent"] => TailwindShadowColor::parse_with_opacity(pattern, arbitrary, opacity)?.boxed(),
            ["color", rest @ ..] => TailwindShadowColor::parse_with_opacity(rest, arbitrary, opacity)?.boxed(),
            // Check if it's a themed color pattern (e.g., ["cyan", "500"])
            [_name, weight] if weight.parse::<u32>().is_ok() => {
                // This is a color like cyan-500
                TailwindShadowColor::parse_with_opacity(pattern, arbitrary, opacity)?.boxed()
            },
            // https://tailwindcss.com/docs/box-shadow-color
            _ => TailwindShadow::parse(pattern, arbitrary, false)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn box_adaptor(str: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/box-decoration-break
            ["decoration" | "break", rest @ ..] => TailwindBoxDecoration::parse(rest, arbitrary)?.boxed(),
            ["clone"] => TailwindBoxDecoration::from("clone").boxed(),
            ["slice"] => TailwindBoxDecoration::from("slice").boxed(),
            // https://tailwindcss.com/docs/box-sizing
            ["border"] => TailwindBoxSizing::from("border-box").boxed(),
            ["content"] => TailwindBoxSizing::from("content-box").boxed(),
            ["sizing", rest @ ..] => TailwindBoxSizing::parse(rest, arbitrary)?.boxed(),
            _ => return syntax_error!("Unknown box instructions: {}", str.join("-")),
        };
        Ok(out)
    }

    #[inline]
    fn backdrop_adaptor(str: &[&str], arbitrary: &TailwindArbitrary, negative: Negative) -> Result<Box<dyn TailwindInstance>> {
        let out = match str {
            // https://tailwindcss.com/docs/backdrop-blur
            ["blur", rest @ ..] => TailwindBlur::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-brightness
            ["brightness", rest @ ..] => TailwindBrightness::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-contrast
            ["contrast", rest @ ..] => TailwindContrast::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-grayscale
            ["grayscale", rest @ ..] => TailwindGrayscale::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-hue-rotate
            ["hue", "rotate", rest @ ..] => TailwindHueRotate::parse(rest, arbitrary, true, negative)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-invert
            ["invert", rest @ ..] => TailwindInvert::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-opacity
            ["opacity", rest @ ..] => TailwindOpacity::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-saturate
            ["saturate", rest @ ..] => TailwindSaturate::parse(rest, arbitrary, true)?.boxed(),
            // https://tailwindcss.com/docs/backdrop-sepia
            ["sepia", rest @ ..] => TailwindSepia::parse(rest, arbitrary, true)?.boxed(),
            _ => return syntax_error!("Unknown backdrop instructions: {}", str.join("-")),
        };
        Ok(out)
    }
    #[inline]
    fn caption_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        // https://tailwindcss.com/docs/caption-side
        Ok(TailwindCaptionSide::parse(pattern, arbitrary)?.boxed())
    }
    #[inline]
    fn table_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        let out = match pattern {
            // https://tailwindcss.com/docs/display#flex
            [] if arbitrary.is_none() => TailwindDisplay::from("table").boxed(),
            ["caption"] => TailwindDisplay::from("table-caption").boxed(),
            ["cell"] => TailwindDisplay::from("table-cell").boxed(),
            ["column"] => TailwindDisplay::from("table-column").boxed(),
            ["column", "group"] => TailwindDisplay::from("table-column-group").boxed(),
            ["footer", "group"] => TailwindDisplay::from("table-footer-group").boxed(),
            ["header", "group"] => TailwindDisplay::from("table-header-group").boxed(),
            ["row", "group"] => TailwindDisplay::from("table-row-group").boxed(),
            ["row"] => TailwindDisplay::from("table-row").boxed(),
            // https://tailwindcss.com/docs/table-layout
            _ => TailwindTableLayout::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
    
    #[inline]
    fn inset_ring_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        use crate::{TailwindInsetRingWidth, TailwindInsetRingColor};
        
        let out = match pattern {
            // inset-ring width utilities (inset-ring, inset-ring-0, inset-ring-1, inset-ring-2, inset-ring-4)
            [] => TailwindInsetRingWidth::parse(&[], arbitrary)?.boxed(),
            ["0"] | ["1"] | ["2"] | ["4"] => TailwindInsetRingWidth::parse(pattern, arbitrary)?.boxed(),
            
            // Arbitrary values are handled with the empty pattern above when arbitrary.is_some()
            
            // inset-ring color utilities (inset-ring-red-500, inset-ring-blue-600, etc.)
            _ => TailwindInsetRingColor::parse(pattern, arbitrary)?.boxed(),
        };
        Ok(out)
    }
    #[inline]
    fn transform_adaptor(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Box<dyn TailwindInstance>> {
        use crate::{TailwindTransformBase, TailwindTransformStyle};
        
        let out = match pattern {
            // Transform base utilities (transform, transform-none, transform-gpu, transform-cpu)
            [] | ["none"] | ["gpu"] | ["cpu"] => TailwindTransformBase::parse(pattern, arbitrary)?.boxed(),
            
            // Transform style utilities (transform-flat, transform-preserve-3d)
            ["flat"] | ["preserve", "3d"] => TailwindTransformStyle::parse(pattern, arbitrary)?.boxed(),
            
            _ => return syntax_error!("Unknown transform instructions: {}", pattern.join("-")),
        };
        Ok(out)
    }
}
