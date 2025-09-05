# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Tailwind-RS is a Rust implementation of a Tailwind CSS-style utility-first CSS framework. It provides JIT (Just-In-Time) and AOT (Ahead-Of-Time) compilation of Tailwind utility classes into CSS, offering a high-performance alternative to JavaScript-based Tailwind CSS processing.

## Common Development Commands

### Building and Testing
```bash
# Run all tests
cargo test --no-fail-fast

# Run tests for a specific crate
cargo test -p tailwind-to-css
cargo test -p tailwind-rs
cargo test -p tailwind-cli

# Run a specific test
cargo test test_html_class_preservation_single_modifier

# Build the project
cargo build
cargo build --release

# Format code (uses rustfmt with custom config)
cargo fmt

# Lint with clippy
cargo clippy
```

### Working with Specific Crates
```bash
# Build/test the core CSS library (no I/O)
cargo build -p tailwind-css
cargo test -p tailwind-css

# Build/test the HTML processor
cargo build -p tailwind-rs  
cargo test -p tailwind-rs

# Build the CLI tool
cargo build -p tailwind-cli --release
```

## Architecture

### Workspace Structure
This is a Cargo workspace with multiple crates:

- **`tailwind-ast/`** - Parser and AST using Yggdrasil grammar (`.ygg` files)
- **`tailwind-to-css/`** - Core CSS generation library (no I/O, pure transformation)
- **`tailwind-rs/`** - HTML processing and configuration layer
- **`tailwind-cli/`** - Command-line interface
- **`tailwind-error/`** - Centralized error handling
- **`tailwind-macro/`** - Procedural macros for compile-time processing
- **`tailwind-show/`** - Web showcase/playground

### Core Library Usage (`tailwind-to-css`)
```rust
use tailwind_css::TailwindBuilder;

let mut tw = TailwindBuilder::default();
// For inline styles
let css = tw.inline("p-4 bg-blue-500");
// For tracking classes
tw.trace("hover:bg-red-500", false);
let bundle = tw.bundle();
```

### HTML Processing (`tailwind-rs`)
```rust
use tailwind_rs::{CLIConfig, CssInlineMode};

let mut config = CLIConfig::default();
let (html, css) = config.compile_html(input_html, &mut builder)?;
```

## Current Development Focus

### Modifier Support (feature/modifier-support branch)
The project is implementing comprehensive modifier support including:
- Pseudo-classes: `hover:`, `focus:`, `active:`, `visited:`
- Responsive breakpoints: `sm:`, `md:`, `lg:`, `xl:`, `2xl:`
- State modifiers: `first:`, `last:`, `odd:`, `even:`
- Dark mode: `dark:`
- Combined modifiers: `sm:hover:`, `lg:focus:`

Test coverage in `projects/tailwind-rs/tests/modifier_acceptance_tests.rs`

## Critical Requirements

### Idempotent Property (ABSOLUTELY CRITICAL)
**THIS IS NON-NEGOTIABLE**: The Tailwind implementation MUST preserve the idempotent property. This means:
- Applying the same transformation multiple times produces the same result as applying it once
- `tw.trace(class)` followed by `tw.trace(class)` must not duplicate CSS rules
- Non-tailwind classes MUST be preserved in the same order as they originally appeared
- All utility classes must be deterministic and consistent
- The entire endeavor is moot without this property

**Testing Requirements**:
- The idempotent property test MUST pass for all utilities
- Random property testing must cover ALL supported utilities
- No utility can be considered complete without idempotent validation

**Idempotency Test Locations**:
- Main idempotency test: `projects/tailwind-rs/tests/test_idempotent.rs`
- Arbitrary value tests: `projects/tailwind-to-css/tests/test_arbitrary_comprehensive.rs`
- When adding new modules, MUST add test cases to verify idempotent behavior
- Pattern: trace the same class multiple times, verify CSS is only generated once

**Why This Matters**:
- We MUST allow consumers of this crate to re-process an already processed html/js/.. file to extract classes
- Critical for production reliability

## Adding New Utility Modules

When implementing a new utility module, follow this checklist:

1. **Create the module** in appropriate category under `tailwind-to-css/src/modules/`
2. **Add to parent mod.rs** exports
3. **Add to resolver** in `src/systems/instruction/resolver.rs`
4. **Create dedicated test file** in `tests/`
5. **Add to idempotency tests** in `projects/tailwind-rs/tests/test_idempotent.rs`
6. **Add arbitrary value tests** if applicable in `projects/tailwind-to-css/tests/test_arbitrary_comprehensive.rs`
7. **Documentation**: Create readme.md with usage examples (use ````rust` with comments, not executable doctests)

## Module Organization

CSS utilities are organized by category in `tailwind-to-css/src/modules/`:
- **Layout**: `layouts/` - display, position, overflow, z-index
- **Flexbox/Grid**: `flexbox/` - flex properties, grid, gap
- **Spacing**: `spacing/` - margin, padding, space-between
- **Sizing**: `sizing/` - width, height, min/max sizes
- **Typography**: `typography/` - font, text, alignment
- **Background**: `background/` - colors, gradients, images
- **Borders**: `borders/` - border width, color, radius
- **Effects**: `effects/` - shadows, opacity, blend modes
- **Transforms**: `transforms/` - rotate, scale, translate
- **Transitions**: `transition/` - animations, transitions

## Development Environment

### Rust Configuration
- **Toolchain**: Nightly Rust (per CI configuration)
- **Format**: Custom rustfmt.toml (128 char width, "Two" version)
- **Profile**: Dev builds with `opt-level = 1` for faster iteration
- **Release**: LTO enabled, panic=abort for smallest binaries

### Parser Development
The project uses Yggdrasil for grammar-based parsing:
- Grammar files: `projects/tailwind-ast/grammars/*.ygg`
- Parser generates AST from Tailwind class strings
- Modifications to grammar require regenerating parser code

## Testing Approach

### Test Organization
- Unit tests in `src/` modules with `#[cfg(test)]`
- Integration tests in `tests/` directories
- Acceptance tests for modifier support in `modifier_acceptance_tests.rs`

### Key Test Patterns
```rust
// Test HTML class generation (some are modified)
let mut tw = TailwindBuilder::default();
let output = tw.trace("hover:bg-blue-500", false).unwrap();
assert_eq!(output, "hover:bg-blue-500");

// Test CSS generation
let css = tw.bundle().unwrap();
assert!(css.contains(".hover\\:bg-blue-500:hover"));
```

## Performance Considerations

- Uses `xxhash-rust` for fast hashing
- `tl` crate for efficient HTML parsing
- `lightningcss` for CSS minification
- Itertools for efficient iteration patterns
- Profile-guided optimizations in release builds
