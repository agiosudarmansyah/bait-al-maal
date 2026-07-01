# Error Handling Guidelines

## Philosophy

Errors are expected failures, not bugs.

Every layer is responsible only for the failures it understands.

If a layer cannot recover from or meaningfully translate an error, it should propagate the error to its caller.

Panics are reserved for programmer mistakes or impossible states.

---

## Layer Responsibilities

### Database Layer

Responsible for:

- Opening database connections
- Executing SQL
- Managing transactions

Returns:

- `libsql::Error`

Should NOT:

- Validate business rules
- Know about UI
- Know about business entities

---

### Repository Layer

Responsible for:

- CRUD operations
- Mapping database rows into entities
- Translating infrastructure errors into domain errors when appropriate

Examples:

- Database failure
- Row mapping failure
- Account not found (if treated as a repository concern)

Should NOT:

- Validate business rules
- Show user-facing messages

---

### Service Layer

Responsible for:

- Business rules
- Domain validation
- Coordinating repositories
- Business workflows

Examples:

- Negative balance
- Empty account name
- Duplicate account
- Invalid transfer
- Budget exceeded

Should NOT:

- Execute SQL directly
- Know about Android UI

---

### FFI Layer

Responsible for:

- Converting Rust types into FFI-safe types
- Translating Rust errors into errors understandable by Kotlin

Should NOT:

- Implement business rules

---

### Kotlin Layer

Responsible for:

- User experience
- User-friendly error messages
- Retry actions
- Dialogs
- Snackbars
- Logging and telemetry

Should NOT:

- Know database implementation details
- Know libsql internals

---

## Propagation

A layer should propagate an error when:

- It cannot recover from it.
- It cannot provide additional business meaning.
- The caller is better suited to decide what to do.

Use `?` whenever propagation is the correct behavior.

---

## Translation

A layer may translate an error when:

- It exposes a different abstraction.
- It wants to hide implementation details.
- It wants to provide domain-specific meaning.

Example:

libsql::Error

↓

AccountError::Database

---

## Handling

A layer handles an error when it can recover or make a meaningful decision.

Examples:

- Retry an operation
- Return a default value
- Display a Snackbar
- Log telemetry
- Abort startup

---

## Panic Checklist

Use `panic!()` only when:

- An internal invariant is violated.
- An impossible state is reached.
- A programmer bug has occurred.
- Required startup configuration is missing.

Do NOT panic for:

- Invalid user input
- Missing database rows
- Network failures
- File I/O failures
- Database failures
- Permission errors

These should return `Result`.
