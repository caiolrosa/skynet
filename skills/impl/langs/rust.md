# Rust Guidelines

## Ownership & Borrowing

- Prefer borrowing (`&T`, `&mut T`) over transferring ownership when the caller still needs the value
- Use `clone()` only when necessary; prefer references to avoid unnecessary allocations
- Keep lifetimes as simple as possible; let the compiler elide them when it can
- Use `Cow<'_, T>` (e.g., `Cow<'_, str>`, `Cow<'_, [u8]>`, `Cow<'_, Path>`) when a function may or may not need to allocate; useful in both parameter and return position

## Error Handling

- Use `Result<T, E>` for recoverable errors; reserve `panic!` for truly unrecoverable situations
- Define custom error types with `thiserror` or manual `impl`; avoid stringly-typed errors
- Use `?` for simple propagation; use `match` when you need variant-specific handling (e.g., matching on `io::ErrorKind`)
- Use `anyhow` in application code for ergonomic error handling; use `thiserror` in library code for typed errors
- Use `.context()` / `.with_context()` from `anyhow` to add human-readable context to errors as they propagate

## Types & Data

- Use `enum` with variants for modeling distinct states; prefer over boolean flags or stringly-typed state
- Use `struct` with named fields; avoid tuple structs unless the meaning is obvious (e.g., newtype pattern)
- Derive `Debug` on all types. Derive `Clone`, `PartialEq`, `Eq`, `Hash` when semantically meaningful — avoid `Clone` on large types where it hides expensive copies, and avoid `PartialEq` on types with floats or interior mutability
- Use `Option<T>` for values that may be absent; avoid sentinel values like `-1` or empty strings

## Pattern Matching

- Use `match` exhaustively; avoid wildcard `_` catch-all unless truly appropriate
- Use `if let` / `while let` for single-variant matching
- Destructure in match arms to access inner values directly

## Iterators

- Prefer iterator chains (`.map()`, `.filter()`, `.flat_map()`, `.collect()`) over manual `for` loops with index access
- Avoid indexing into `Vec` when iterating; use iterators directly or `.enumerate()` when the index is needed
- Use `.iter()` / `.iter_mut()` for borrowing, `.into_iter()` for consuming ownership

## String Handling

- Accept `&str` in function parameters; return `String` when the function allocates
- Use `format!` for constructing strings; avoid manual `push_str` concatenation chains

## Functions & Methods

- Keep functions small and single-purpose
- Use `impl Trait` in argument position for flexibility (`fn process(reader: impl Read)`); note this hides the concrete type from callers and docs
- Prefer methods on types (`impl MyType`) over free functions when the function is closely tied to the type

## Traits & Generics

- Use traits to define shared behavior; choose between static dispatch (generics) and dynamic dispatch (`dyn Trait`) based on the use case — `dyn Trait` is appropriate for heterogeneous collections, plugin architectures, and reducing compile times
- Keep generic type parameters to a minimum; avoid deeply nested generics
- Implement `From` (not `Into` directly — `From` gives you `Into` for free), `Display`, and `Default` when they make sense for your type

## Concurrency

- Prefer message-passing (`mpsc`, `crossbeam` channels) over shared state for communication between threads
- When shared state is needed, use `Arc<Mutex<T>>` or `Arc<RwLock<T>>`; consider `parking_lot` for better performance; use atomics for simple counters and flags
- Use `Send` and `Sync` trait bounds to enforce thread safety at compile time
- Use `tokio` for async I/O (the ecosystem standard); avoid mixing blocking and async code — use `spawn_blocking` or `block_in_place` for blocking work inside async contexts

## Naming

- `snake_case` for functions, methods, variables, modules, and crate names
- `PascalCase` for types, traits, and enum variants
- `UPPER_SNAKE_CASE` for constants and statics
- Prefix unused variables with `_` to silence warnings

## Project Structure

- Keep `main.rs` thin; extract logic into `lib.rs` and modules — this binary/library split enables unit testing of application logic
- Use modules (`mod`) to organize code by concern
- Prefer explicit `pub` visibility; keep internals private by default
- Run `cargo clippy -- -D warnings` as a baseline; treat clippy lints as mandatory
- Use `cfg(test)` modules for unit tests alongside the code they test
- Place integration tests in the `tests/` directory

## Unsafe

- Avoid `unsafe` unless necessary; when used, encapsulate it behind a safe API
- Document every `unsafe` block with a `// SAFETY:` comment explaining why the invariants are upheld
- Prefer safe abstractions from `std` or well-audited crates over hand-rolled `unsafe`

## Dependencies

- Evaluate crate quality before adding: maintenance activity, audit status, transitive dependency count
- Prefer `std` when sufficient; avoid pulling in a crate for something the standard library handles
