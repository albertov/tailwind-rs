# User Select Module

This module implements the Tailwind CSS user-select utilities, which control whether users can select text in an element.

## Supported Utilities

- `select-none` - Prevents text selection (user-select: none)
- `select-text` - Allows text selection (user-select: text)
- `select-all` - Selects all text on click (user-select: all)
- `select-auto` - Uses default browser behavior (user-select: auto)

## Arbitrary Values

The module also supports arbitrary values using square bracket notation:
- `select-[contain]` - Sets user-select: contain
- `select-[inherit]` - Sets user-select: inherit
- `select-[initial]` - Sets user-select: initial
- `select-[revert]` - Sets user-select: revert
- `select-[unset]` - Sets user-select: unset

## Examples

```html
<!-- Prevent text selection -->
<div class="select-none">
  This text cannot be selected
</div>

<!-- Allow text selection -->
<div class="select-text">
  This text can be selected normally
</div>

<!-- Select all text on click -->
<div class="select-all">
  Click anywhere to select all this text
</div>

<!-- With modifiers -->
<div class="hover:select-none">
  Text selection disabled on hover
</div>
```

## Browser Support

The user-select property is well-supported in modern browsers. For older browsers, vendor prefixes may be needed:
- `-webkit-user-select` for Safari
- `-moz-user-select` for older Firefox
- `-ms-user-select` for older IE/Edge

## Implementation Notes

This module uses the `keyword_instance!` macro which automatically:
1. Generates a `TailwindInstance` implementation
2. Maps the StandardValue to the appropriate CSS property
3. Handles arbitrary values through the StandardValue parser