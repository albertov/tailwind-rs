# Line Clamp

The `line-clamp` utilities control the number of lines of text displayed in an element.

## Usage

```rust
# use tailwind_css::*;
// Standard line clamp values
// Usage: class="line-clamp-3"
// Generates: overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 3;

// Remove line clamping
// Usage: class="line-clamp-none"
// Generates: overflow: visible; display: block; -webkit-box-orient: horizontal; -webkit-line-clamp: none;
```

## Available Values

- `line-clamp-none` - Remove line clamping
- `line-clamp-1` - Clamp to 1 line
- `line-clamp-2` - Clamp to 2 lines
- `line-clamp-3` - Clamp to 3 lines
- `line-clamp-4` - Clamp to 4 lines
- `line-clamp-5` - Clamp to 5 lines
- `line-clamp-6` - Clamp to 6 lines
- `line-clamp-[n]` - Arbitrary value for custom line counts