# Extractor - AI Coding Agent Instructions

## Project Overview
A Rust-based gaming data extraction and filtering tool. Processes transaction JSON files from multiple gaming platforms (Hacksaw, Octoplay, EnjoyGaming), filters by game-specific logic, and outputs results to JSON files.

## Architecture

### Core Module Structure
- **`lib.rs`**: Exports `storage` and `games` modules (no implementation)
- **`storage.rs`**: Handles I/O operations - wraps `walkdir` + `serde_json`
  - `load_transactions(path)` → reads JSON files recursively from directory or single file, with progress bar
  - `save_content(path, content)` → writes JSON to file, creates parent directories automatically
- **`games/`**: Provider-specific extraction logic
  - `games/{provider}/mod.rs`: Test module entry point (imports game module, defines `#[cfg(test)]` tests)
  - `games/{provider}/{game}.rs`: Core extraction logic with filter-specific functions

### Key Data Flow
1. Game-specific modules call `load_transactions()` to fetch raw transaction data
2. Iterate through transactions, applying game-specific filters (board state, win conditions, symbols)
3. Push matching transactions to `Vec<Value>`
4. Format results and call `save_content()` to persist filtered output as JSON

### Cross-Cutting Patterns
- **Error Handling**: Uses `anyhow::Result<T>` (see `main.rs`), but storage functions use `unwrap_or_default()` for resilience
- **Progress Feedback**: All I/O operations use `indicatif::ProgressBar` with custom templates
- **JSON Manipulation**: Raw `serde_json::Value` (dynamic typing) - no struct-based deserialization currently

## Module Extension Pattern
When adding a new game provider or game:

1. **Create module structure**: `src/games/{provider}/{game}.rs`
2. **Define extraction function**: `pub fn extract_*() { ... }`
3. **Import in mod.rs**: `pub mod {game};`
4. **Add test in mod.rs**:
   ```rust
   #[cfg(test)]
   mod tests_{game} {
       use crate::games::{provider}::{game};
       #[test] fn test_extract() { {game}::extract_by_filter(); }
   }
   ```
5. **Reference data paths**: Use relative paths like `../data/{provider}/{game}/transactions/`

## Dependencies & Their Use
- `serde_json`: JSON parsing and Value manipulation
- `anyhow`: Error context (wrap with `?` operator)
- `indicatif`: Progress bars for long-running I/O
- `walkdir`: Recursive directory traversal (handles deeply nested transaction files)
- `serde`: Serialization foundation (required by serde_json)

## Build & Test
- **Build**: `cargo build` (standard)
- **Run**: `cargo run` (main.rs currently empty - test via `cargo test`)
- **Test**: `cargo test` - runs all game extraction tests defined in `games/*/mod.rs`

## Code Style Notes
- Naming: Parameter prefixes indicate intent (`a_location`, `l_transactions`) - follow this style
- Type hints: Explicit turbofish operators sometimes used (e.g., `serde_json::from_str::<T>(...)`)
- String formatting: Prefer `.to_owned()` + `+` over `format!()` (inconsistent, but established pattern)
- Closures in iterators: Common to chain `.filter_map(|e| ...)`, `.filter(|e| ...)`, `.map(|e| ...)`

## File Locations
- Data input: `../data/{provider}/{game}/transactions/*.json`
- Data output: `../data/{provider}/{game}/settings/filtred.json`
- Tests: Each game's `mod.rs` contains test definitions
