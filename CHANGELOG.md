# CHANGELOG

## 0.0.0

- Initial commit

## 0.1.0

- Moved everything backend related into a new folder from src-tauri/ to core/rust/ for better organizing
- Added necessary features (account, budget, etc.) and infrastructures (database, auth, sync)
- Crates and modules have been created and tidied (3 crates in total, rust/core/ have been tidied up)
- As for features:
  - Account: added entity, repository, and service
  - Budget: only added entity
  - Transaction: only added entity
  - Categories: only added entity
  - User: only added entity
- Added database struct and implementation
