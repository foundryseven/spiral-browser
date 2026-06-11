# Spiral Browser — Testing Guide

## Test Levels

### Unit Tests
Location: `crates/spiral-{name}/src/*.rs` (inline `#[cfg(test)]`)

```bash
# Run all unit tests
cargo test

# Run unit tests for a specific crate
cargo test spiral-core
cargo test spiral-layout
```

### Integration Tests
Location: `crates/spiral-{name}/tests/*.rs`

```bash
# Run integration tests for a crate
cargo test --package spiral-html
```

### Workspace Tests
```bash
# Run all tests across all crates
cargo test --workspace
```

### Web Platform Tests (WPT)
Location: `tests/wpt/`

```bash
# Run WPT tests (Phase 5)
cd tests/wpt
./run-wpt.sh
```

## Test Commands

```bash
# Full test suite
cargo test --workspace

# With output
cargo test --workspace -- --nocapture

# Run specific test
cargo test test_box_model

# Run tests matching pattern
cargo test layout::tests

# Run ignored tests
cargo test -- --ignored

# Run all tests including ignored
cargo test -- --include-ignored
```

## Writing Tests

### Unit Test Template
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = my_function(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_edge_case() {
        let result = my_function(empty_input);
        assert!(result.is_err());
    }
}
```

### Integration Test Template
```rust
// tests/integration_test.rs
use spiral_core::TabId;

#[test]
fn test_cross_crate_flow() {
    // Test that spiral-html produces correct spiral-dom output
    // when given HTML parsed by spiral-html
}
```

### Test Naming Convention
- `test_{function_name}_{scenario}` — unit tests
- `test_{feature}_{behavior}` — integration tests
- `test_{module}_{edge_case}` — edge case tests

## Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --package spiral-layout

# Run with specific filter
cargo bench "box model"
```

## Coverage

```bash
# Generate coverage report (requires cargo-tarpaulin)
cargo tarpaulin --workspace

# Generate HTML report
cargo tarpaulin --workspace --out Html
```

## CI Test Matrix

| Platform | OS | Test Command |
|----------|-----|-------------|
| Linux | ubuntu-latest | `cargo test --workspace` |
| macOS | macos-latest | `cargo test --workspace` |
| Windows | windows-latest | `cargo test --workspace` |

## Platform-Specific Tests

```rust
#[cfg(target_os = "linux")]
#[test]
fn test_linux_sandbox() {
    // Linux-specific test
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_sandbox() {
    // macOS-specific test
}

#[cfg(target_os = "windows")]
#[test]
fn test_windows_sandbox() {
    // Windows-specific test
}
```

## Performance Tests

Layout benchmarks measure:
- Box model computation time
- Flexbox layout time
- Grid layout time
- Text shaping time
- Display list generation time

Render benchmarks measure:
- Vello scene build time
- GPU execution time
- Frame time (target: <16.67ms for 60fps)

IPC benchmarks measure:
- Message serialization time
- Round-trip latency
- Throughput (messages/second)

## Debugging Tests

```bash
# Run test with RUST_LOG
RUST_LOG=debug cargo test test_name -- --nocapture

# Run test under valgrind (Linux)
valgrind cargo test test_name

# Run test with backtrace
RUST_BACKTRACE=1 cargo test test_name
```
