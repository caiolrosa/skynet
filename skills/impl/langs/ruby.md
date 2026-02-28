# Ruby Guidelines

## Style & Syntax

- Use 2-space indentation
- Always use double-quoted strings (`"hello"`, `"hello #{name}"`); only use single quotes when the string itself contains double quotes
- Use `snake_case` for methods, variables, and file names
- Use `PascalCase` for classes and modules
- Use `UPPER_SNAKE_CASE` for constants
- Prefer `do...end` for multi-line blocks and `{ }` for single-line blocks

## Methods

- Keep methods short and single-purpose
- Use `?` suffix for methods that return booleans (`empty?`, `valid?`)
- Use `!` suffix for methods that mutate state or may raise (`save!`, `destroy!`)
- Prefer keyword arguments for methods with more than 2 parameters
- Omit `return` for implicit returns at the end of a method; use explicit `return` for early exits

## Control Flow

- Prefer `unless` for simple negated conditions (`unless valid?` not `if !valid?`); avoid `unless` with `else` or compound conditions
- Use `if`/`unless` as statement modifiers for simple one-liners (`raise ArgumentError unless valid?`)
- Use `case/when` over long `if/elsif` chains; use `case/in` for pattern matching (Ruby 3.0+)
- Prefer guard clauses over nested conditionals

## Collections

- Use `map`, `select`, `reject`, `find`, `reduce` over manual loops
- Prefer `each` over `for`
- Use `&:method` shorthand when passing a single method call (`names.map(&:upcase)`)
- Use `Hash#fetch` with a default or block instead of `[]` when the key might be missing

## Error Handling

- Rescue specific exceptions, never bare `rescue` — it catches all `StandardError` subclasses, hiding bugs
- Create custom exception classes inheriting from `StandardError` for domain errors
- Use `ensure` for cleanup that must always run
- Avoid using exceptions for flow control

## Classes & Modules

- Use `attr_reader`, `attr_writer`, `attr_accessor` instead of manual getters/setters
- Prefer composition over inheritance
- Use modules for shared behavior (`include` for instance methods, `extend` for class methods)
- Keep class definitions focused; extract concerns into modules when a class grows

## Ruby Idioms

- Use `# frozen_string_literal: true` at the top of every file (Ruby 3.0+); manual `.freeze` is only needed for non-string constants
- Prefer `dig` for safe nested hash/array access (`params.dig(:user, :address, :city)`)
- Use `Struct` or `Data` (Ruby 3.2+) for simple value objects
- Prefer `then`/`yield_self` for method chaining pipelines

## Testing

- Use the project's existing framework; prefer Minitest for libraries and RSpec for applications when starting fresh
- Use `describe`/`context`/`it` blocks (RSpec) or method-named tests (Minitest) to clearly state what is being tested
- Use factories (`factory_bot`) over fixtures for test data; keep factories minimal with only required attributes
- Mock external dependencies at the boundary; avoid mocking the object under test

## Rails (apply only when the project uses Rails)

- Keep controllers and models lean — extract complex business logic into service objects or plain Ruby classes
- Use strong parameters in controllers; never trust raw `params`
- Use scopes for reusable query logic on models
- Use callbacks (`before_action`, `after_create`, etc.) sparingly — prefer explicit service objects for complex business logic
- Use Active Record validations on models; avoid duplicating validation in controllers
- Use `find_by` instead of `where(...).first`; use `find` when the record must exist
- Avoid N+1 queries — use `includes`, `preload`, or `eager_load` for associations
- Use migrations for all schema changes; never modify the database manually
- Use `has_many`/`belongs_to`/`has_one` with `dependent:` option to handle cascading deletes
- Keep business logic in service objects or plain Ruby classes, not in controllers or callbacks
- Use `I18n` for user-facing strings; avoid hardcoded text in views and controllers
- Use partials and helpers to keep views DRY, but avoid logic-heavy helpers — use view components or presenters instead

## Dependencies

- Use Bundler for all dependency management
- Prefer pessimistic version constraints (`~>`) for gems in the Gemfile

## Concurrency

- Be aware of the GVL (Global VM Lock) — CPU-bound work won't parallelize across threads
- Ensure thread safety when using threaded servers like Puma — avoid shared mutable state
