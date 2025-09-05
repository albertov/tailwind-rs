# TailwindBorderSpacing

Controls the spacing between table borders.

## Usage

```rust
use tailwind_css::{TailwindBorderSpacing, TailwindArbitrary};

// Standard spacing values
let spacing = TailwindBorderSpacing::parse(&["2"], &TailwindArbitrary::from("")).unwrap();
let spacing_x = TailwindBorderSpacing::parse(&["x", "4"], &TailwindArbitrary::from("")).unwrap();
let spacing_y = TailwindBorderSpacing::parse(&["y", "8"], &TailwindArbitrary::from("")).unwrap();

// Arbitrary values
let arbitrary = TailwindArbitrary::from("10px");
let custom = TailwindBorderSpacing::parse(&[], &arbitrary).unwrap();
```

## Supported Classes

- `border-spacing-{0-96}` - Sets border spacing for all sides
- `border-spacing-x-{0-96}` - Sets horizontal border spacing
- `border-spacing-y-{0-96}` - Sets vertical border spacing
- `border-spacing-px` - Sets 1px border spacing
- `border-spacing-[value]` - Arbitrary value support

## CSS Properties

- `border-spacing` - Sets the distance between borders of adjacent table cells
- `--tw-border-spacing-x` - CSS variable for horizontal spacing
- `--tw-border-spacing-y` - CSS variable for vertical spacing