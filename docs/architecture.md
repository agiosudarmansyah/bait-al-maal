# Architecture

## Overview

BAIT-AL-MAAL separates platform-specific UI from shared business logic.

```text
Android (Compose)
    │
ViewModel
    │
JNI
    │
Rust Core
    │
SQLite
```

Desktop follows a similar principle using Tauri while reusing the same Rust core.

## Design Goals

* One source of truth for business rules.
* Platform-native user interfaces.
* Shared validation across platforms.
* High runtime performance.
* Maintainable long-term architecture.

## Why Rust?

Rust contains all business logic, domain models, repositories and database access.

Benefits include:

* Memory safety
* Excellent performance
* Shared implementation across platforms

## Why Kotlin + Compose?

Originally the project used Flutter.

The project later migrated to Kotlin + Jetpack Compose because:

* Better Android integration
* Native platform understanding
* Improved debugging experience
* Better alignment with long-term learning goals

The trade-off is maintaining a separate iOS UI in the future.
