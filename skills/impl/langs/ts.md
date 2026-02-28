# JavaScript & TypeScript Guidelines

> These guidelines apply to both JavaScript and TypeScript projects. Loaded for `lang:js` and `lang:ts`.

## Language & Runtime

- Use modern ES2020+ syntax: `const`/`let` (never `var`), arrow functions, template literals, optional chaining (`?.`), nullish coalescing (`??`)
- Prefer `const` by default; use `let` only when reassignment is needed
- Use strict equality (`===` / `!==`), never loose equality

## Functions

- Prefer arrow functions for callbacks and anonymous functions
- Use named function declarations for top-level functions (better stack traces)
- Keep functions small and single-purpose
- Use default parameters instead of manual checks

## Async

- Prefer `async/await` over raw Promises and `.then()` chains
- Always handle errors in async code with `try/catch` or `.catch()`
- Use `Promise.all()` for independent concurrent operations
- Use `Promise.allSettled()` when you need results from all promises regardless of individual failures
- Avoid mixing callbacks and Promises in the same flow

## Modules

- Use ES modules (`import`/`export`) over CommonJS (`require`/`module.exports`) unless the project uses CommonJS
- Use named exports by default; use default exports only for main module entry points
- Use `import type` / `export type` for type-only imports to ensure they're erased at compile time
- Keep imports organized: external dependencies first, then internal modules

## Error Handling

- Throw `Error` objects (or subclasses), never strings or plain objects
- Catch specific errors when possible; avoid empty catch blocks
- Let unexpected errors propagate rather than silently swallowing them

## Data

- Use `Array` methods (`map`, `filter`, `reduce`, `find`) over manual loops when they improve clarity
- Prefer object destructuring and spread syntax for immutable-style updates
- Use `Map`/`Set` when appropriate instead of plain objects/arrays

## Naming

- `camelCase` for variables, functions, and methods
- `PascalCase` for classes and constructor functions
- `UPPER_SNAKE_CASE` for true constants (values that never change across the program)
- Descriptive names that convey purpose; avoid single-letter variables except in short lambdas

## TypeScript-Specific

- Always annotate function parameters; annotate return types on exported/public functions and when inference produces an undesirably wide type
- Prefer `interface` for object shapes that may be extended; use `type` for unions, intersections, and mapped types
- Use `unknown` over `any`; narrow types with type guards before accessing properties
- Prefer `readonly` for properties and arrays that should not be mutated
- Use discriminated unions for state modeling instead of optional fields with implicit relationships
- Avoid type assertions (`as`) unless you can guarantee correctness; prefer type narrowing
- Use `satisfies` to validate a value matches a type without widening it
- Use generics to avoid code duplication, but keep them simple — avoid deeply nested generic types
- Avoid `enum`; prefer `as const` objects for named constant sets and union types (`'a' | 'b'`) for string literals
- Use assertion functions (`asserts x is T`) for runtime validation that also narrows types
- Use `const` type parameters when you need to preserve literal types in generic inference
- Use `using` / `await using` for resources that need deterministic cleanup (replaces manual `try/finally`)

## Testing

- Use the project's existing test framework; when starting fresh, prefer Vitest for Vite-based projects and Jest otherwise
- Co-locate test files next to source files (`foo.test.ts` alongside `foo.ts`) unless the project uses a separate `__tests__/` directory
- Use descriptive test names that state the expected behavior, not the implementation
- Prefer `toEqual` for value comparison and `toBe` for identity/primitive comparison
- Mock external dependencies (network, filesystem, time) at the boundary; avoid mocking internal modules
