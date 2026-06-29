# Decision

Use Rust as the shared business logic layer.

## Context

The application targets multiple platforms.

Duplicating business logic across platforms would increase maintenance costs and risk inconsistent behavior.

## Edict

Business logic, repositories and database access are implemented in Rust.

## Consequences

Positive

* Shared implementation
* Better consistency
* Excellent performance

Negative

* FFI layer required
* Slightly higher initial complexity
