# Shell Script Guidelines

## Shebang & Setup

- Always start scripts with `#!/usr/bin/env bash` for portability
- Use `set -euo pipefail` at the top of every script to fail on errors, unset variables, and pipe failures
  - Be aware that `set -e` does not propagate into subshells, functions used in `if` conditions, or the left-hand side of `&&`/`||`. Test critical error paths explicitly.
- Use `set -x` only for debugging; remove before committing

## Variables

- Always quote variables: `"${var}"` not `$var`
- Use `${var}` braces consistently for clarity
- Use `local` for variables inside functions
- Use `readonly` for variables that should not change
- Use `UPPER_SNAKE_CASE` for exported/environment variables; `lower_snake_case` for local script variables
- Use `${var:-default}` for default values instead of `if` blocks
- Use `${var:?error message}` for required variables instead of manual checks
- Use `${#var}` for string length instead of `wc -c`

## Arrays

- Use arrays for lists of items: `files=("a.txt" "b.txt")`
- Iterate with `for f in "${files[@]}"` (quoted `@`)
- Never parse `ls` output; use globs or `find -print0 | while IFS= read -r -d ''`

## Control Flow

- Use `[[` over `[` for conditionals in bash scripts (supports regex, glob, and avoids word splitting issues)
- Use `if` statements over `&&`/`||` chains for anything more than a single simple command
- Quoting inside `[[ ]]` is optional (no word splitting or globbing occurs), but harmless if you prefer consistency with `[ ]`
- Use `case` statements over long `if/elif` chains

## Arithmetic

- Use `$(( ))` for arithmetic, not `expr` or `let`
- Prefer `(( x > 5 ))` over `[[ $x -gt 5 ]]` for numeric comparisons

## Functions

- Define functions with `function_name() { }` syntax (no `function` keyword)
- Use `local` for all function-internal variables
- Keep functions focused and single-purpose
- Return exit codes (`return 0` for success, `return 1` for failure); use `echo` or stdout for data output
- Functions run in the current shell (not a subshell) unless piped

## Command Execution

- Use `$()` for command substitution, never backticks
- Check command existence with `command -v` before using optional dependencies
- Use `trap` for cleanup on exit, error, or signals (`trap cleanup EXIT`)
- Prefer built-in shell features over external commands when possible (`${var%pattern}` over `sed` for simple substitutions)
- Use process substitution for comparing command outputs: `diff <(cmd1) <(cmd2)`
- Use here-strings to parse variables: `while read ... <<< "${var}"`

## Input & Arguments

- Always validate required arguments at the top of the script
- Use `getopts` or manual parsing for flags; provide a `usage()` function
- Use `"$@"` to pass arguments through; never use unquoted `$*`

## Reading Input

- Use `read -r` to prevent backslash interpretation
- Use `IFS= read -r line` to preserve leading/trailing whitespace
- Use `while IFS= read -r line` for line-by-line file processing

## Output & Logging

- Use `>&2` for error and diagnostic messages (stderr); reserve stdout for data
- Provide meaningful error messages that include what went wrong and what to do about it

## File Operations

- Use `mktemp` for temporary files; clean up with `trap`
- Use `[[ -f "${file}" ]]` for regular files, `[[ -d ]]` for directories, `[[ -e ]]` for any path, `[[ -r ]]`/`[[ -w ]]`/`[[ -x ]]` for permission checks
- Prefer attempting the operation and handling the error over TOCTOU-prone existence checks when possible
- Use `mkdir -p` when creating directories that may already exist

## Portability

- The rules above assume bash (per the `#!/usr/bin/env bash` shebang). When targeting POSIX `sh`/`dash`, avoid bash-specific features (`[[ ]]`, arrays, process substitution, here-strings)
- Use `printf` over `echo` for consistent behavior across platforms when formatting matters
- Test scripts on the target platform; avoid assuming GNU coreutils
