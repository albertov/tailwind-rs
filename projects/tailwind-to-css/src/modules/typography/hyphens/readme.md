# Hyphens

The `hyphens` utilities control how words are hyphenated when text wraps across multiple lines.

## Usage

```rust
# use tailwind_css::*;
// No hyphenation - generates: hyphens: none;
// Usage: class="hyphens-none"

// Manual hyphenation (only at &shy; characters) - generates: hyphens: manual;
// Usage: class="hyphens-manual"

// Automatic hyphenation - generates: hyphens: auto;
// Usage: class="hyphens-auto"
```

## Available Values

- `hyphens-none` - Prevent hyphenation
- `hyphens-manual` - Only hyphenate at soft hyphen characters (&shy;)
- `hyphens-auto` - Automatically hyphenate words based on language rules
- `hyphens-[value]` - Arbitrary value for custom hyphenation settings