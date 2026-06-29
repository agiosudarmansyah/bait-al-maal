# Decision

Adopt a monorepo structure.

## Structure

```text
apps/
rust/
docs/
cloud/
```

Each application owns its platform-specific code while Rust remains platform-independent.
