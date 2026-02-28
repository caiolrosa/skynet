# CSS Guidelines

## Selectors

- Prefer class selectors over element or ID selectors
- Keep specificity low; avoid deeply nested selectors (max 3 levels)
- Never use `!important` unless overriding third-party styles with no other option
- Use meaningful class names that describe purpose, not appearance (`.error-message` not `.red-text`)

## Nesting

- Use native CSS nesting (`&`) for related selectors to co-locate styles
- Keep nesting shallow (max 2 levels) to avoid specificity creep
- Prefer nesting for pseudo-classes/elements and direct children, not for deep descendants

## Layout

- Use Flexbox for one-dimensional layouts (rows or columns)
- Use CSS Grid for two-dimensional layouts (rows and columns together)
- Avoid floats for layout; reserve them for wrapping text around images
- Use `gap` instead of margins between flex/grid children

## Box Model

- Use `box-sizing: border-box` globally
- Prefer `padding` for internal spacing and `margin` for external spacing
- Prefer logical properties (`inline-size`, `block-size`, `margin-inline`, `padding-block`) over physical ones (`width`, `height`, `margin-left`) for layout that adapts to writing direction
- Avoid negative margins unless solving a specific alignment problem

## Responsive Design

- Use relative units (`rem`, `em`, `%`, `vw`, `vh`) over fixed `px` where appropriate
- Use `px` for borders, shadows, and small fixed-size elements
- Design mobile-first: base styles for small screens, `min-width` media queries for larger breakpoints
- Prefer `@container` queries for component-level responsive behavior; reserve `@media` for page-level layout shifts
- Use `clamp()` for fluid typography and spacing when appropriate

## Colors & Theming

- Use CSS custom properties (`--var`) for colors, spacing, and repeated values
- Define theme variables on `:root` or a top-level container
- Use `currentColor` to inherit text color in borders, SVGs, and decorative elements

## Typography

- Set a base `font-size` on `html` or `:root` and use `rem` for all other sizes
- Limit the number of font weights and sizes to maintain visual consistency
- Use `line-height` without units (e.g., `1.5` not `1.5rem`) for proportional scaling

## Animations & Transitions

- Prefer animating composited properties (`transform`, `opacity`, `filter`) for performance
- Avoid animating layout-triggering properties (`width`, `height`, `margin`, `padding`) in hot paths
- Use `prefers-reduced-motion` to respect user accessibility settings
- Prefer CSS transitions for simple state changes; use `@keyframes` for multi-step animations

## Accessibility

- Always provide visible focus styles (`:focus-visible`) for interactive elements
- Ensure sufficient color contrast ratios (4.5:1 for normal text, 3:1 for large text)
- Do not rely on color alone to convey information

## Layers

- Use `@layer` to manage specificity between resets, base styles, components, and utilities

## Organization

- Group related properties together: positioning, display, box model, typography, visual, misc
- Use consistent naming conventions (BEM, utility classes, or whatever the project uses)
- Avoid inline styles; keep styles in stylesheets or CSS modules

## Methodology

- Follow the project's existing approach (CSS Modules, Tailwind, styled-components, etc.)
- When starting fresh, prefer CSS Modules or scoped styles for component isolation