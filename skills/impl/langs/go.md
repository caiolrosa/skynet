# Go Guidelines

## Style & Formatting

- Use `gofmt`/`goimports` — never argue about formatting
- Use `camelCase` for unexported identifiers, `PascalCase` for exported
- Use all-caps for acronyms in exported names: `UserID`, `HTTPClient`. In unexported names, only the first word is lowercased: `userID`, `httpClient`
- Keep names short and descriptive; avoid stuttering (`user.UserName` → `user.Name`)
- Avoid underscores and hyphens in package names

## Error Handling

- Always check returned errors; never use `_` to discard them unless explicitly justified
- Return errors to the caller; handle them at the appropriate level
- Wrap errors with context using `fmt.Errorf("doing X: %w", err)` to preserve the error chain
- Use sentinel errors (`var ErrNotFound = errors.New(...)`) or custom error types for errors callers need to inspect
- Use `errors.Is` and `errors.As` for error comparison, not string matching

## Functions

- Keep functions short and single-purpose
- Return early on errors (guard clauses) to reduce nesting
- Accept interfaces, return concrete types
- Use named return values sparingly — only when they improve readability

## Structs & Interfaces

- Define interfaces where they are consumed, not where they are implemented
- Keep interfaces small — one or two methods is ideal
- Use struct embedding for composition, not inheritance
- Use pointer receivers when the method mutates state or the struct is large; value receivers otherwise
- Design structs so the zero value is useful (e.g., `sync.Mutex`, `bytes.Buffer`). When zero values aren't safe, provide a `New...` constructor and document that it must be used

## Enums

- Use `iota` for enums
- Start with an explicit zero value that represents "unset" or "unknown" to catch uninitialized fields
- Always add a `String()` method

## Concurrency

- Don't reach for goroutines when sequential code is sufficient — concurrency adds complexity, justify it with actual need (I/O parallelism, CPU-bound work, timeouts)
- Always ensure goroutines can terminate; avoid leaking them
- Use `sync.WaitGroup` to wait for goroutine completion
- Use `context.Context` for cancellation and timeouts; pass it as the first parameter
- Never store `context.Context` in a struct field
- Never use `context.TODO()` in production code — it's a placeholder for refactoring
- Derive child contexts with `context.WithCancel`/`WithTimeout` rather than passing long-lived contexts deep into call stacks
- Protect shared state with `sync.Mutex` or `sync.RWMutex`; prefer channels when they simplify the design
- Use `sync.Once` for lazy initialization

## Packages & Modules

- One package per directory; package name should match directory name by convention
- Keep `main` packages thin — extract logic into importable packages
- Avoid circular dependencies; structure packages by responsibility
- Use internal packages to restrict visibility within a module
- Avoid `init()` functions — they make testing harder, create hidden dependencies, and can cause import-order surprises. Prefer explicit initialization

## Testing

- Use `package foo_test` for black-box testing of exported APIs; use `package foo` in test files only when testing unexported internals
- Use table-driven tests for multiple input/output scenarios
- Use `t.Helper()` in test helper functions for better error reporting
- Prefer standard library testing; use `testify` only if the project already depends on it

## Common Patterns

- Use `defer` for cleanup (closing files, unlocking mutexes) immediately after acquiring the resource
- Avoid `defer` inside loops — resources won't be released until the function returns. Extract the loop body into a separate function if cleanup is needed per iteration
- Use `io.Reader` and `io.Writer` interfaces for streaming data
- Use `struct{}` as the element type for sets (`map[string]struct{}`)
- Prefer `strings.Builder` over string concatenation in loops

## Generics

- Use generics for type-safe data structures and utility functions
- Don't use generics when an interface would be simpler or when the type set is small enough for concrete implementations

## Logging

- Use `log/slog` (Go 1.21+) for structured logging
- Avoid `log.Fatal`/`log.Panic` outside `main` — they call `os.Exit`/`panic` and bypass deferred cleanup

## Build & Linting

- Run `go vet`, `staticcheck`, and `golangci-lint` if available
- Keep linter configuration consistent across the project

## Dependency Management

- Run `go mod tidy` to keep `go.mod` and `go.sum` clean
- Minimize external dependencies — prefer the standard library when it's sufficient
- Vet new dependencies for maintenance status, license compatibility, and transitive dependency weight
