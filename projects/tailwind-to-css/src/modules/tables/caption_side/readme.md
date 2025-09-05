# TailwindCaptionSide

Controls the position of a table caption.

## Usage

```rust
use tailwind_css::{TailwindCaptionSide, TailwindArbitrary};

// Standard positions
let top = TailwindCaptionSide::parse(&["top"], &TailwindArbitrary::from("")).unwrap();
let bottom = TailwindCaptionSide::parse(&["bottom"], &TailwindArbitrary::from("")).unwrap();

// Arbitrary values
let arbitrary = TailwindArbitrary::from("block-start");
let custom = TailwindCaptionSide::parse(&[], &arbitrary).unwrap();
```

## Supported Classes

- `caption-top` - Places the caption at the top of the table
- `caption-bottom` - Places the caption at the bottom of the table
- `caption-[value]` - Arbitrary value support

## CSS Properties

- `caption-side` - Specifies the position of a table caption