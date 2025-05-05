# Code Status Utilities

Technical utilities for code quality tracking and documentation in Rust projects.

## Components

### code-status-macros

Procedural macros for annotating code with development status information.

These macros add compile-time markers without affecting runtime behavior:

```rust
use code_status_macros::*;

#[untested]
#[needs("error handling")]
fn parse_config(input: &str) -> Config {
    // ...
}
```

### code-status-scanner

CLI tool to find and report on macro annotations.

```
code-status-scanner -p src/ summary
```

## Installation

```sh
# Add macros to your project
cargo add code-status-macros

# Install scanner globally
cargo install code-status-scanner
```

## Macro Reference

### Code Quality Markers

| Macro | Purpose | Arguments |
|-------|---------|-----------|
| `#[untested]` | No tests | None |
| `#[includes_unwrap]` | Contains unwrap() | None |
| `#[needs("detail")]` | Needs improvement | Required string |
| `#[perf_critical]` | Performance-critical | None |
| `#[security_sensitive]` | Security concerns | None |
| `#[unsafe_usage("reason?")]` | Uses unsafe code | Optional string |
| `#[no_clippy("reason")]` | Suppressed lints | Required string |
| `#[complexity("details")]` | Algorithmic complexity | Required string |
| `#[allocation_heavy("details?")]` | Memory allocation concerns | Optional string |
| `#[panic_path("scenario")]` | Documents panic paths | Required string |

### Review & Future Work Markers

| Macro | Purpose | Arguments |
|-------|---------|-----------|
| `#[needs_review]` | Requires review | None |
| `#[temporary]` | Temporary solution | None |
| `#[assumptions("detail")]` | Documents assumptions | Required string |
| `#[revisit_in("version")]` | Future work needed | Required string |
| `#[dependency_sensitive]` | Sensitive to dependencies | None |
| `#[platform_specific("platform")]` | Platform-specific code | Required string |
| `#[feature_gated("feature")]` | Feature flag dependent | Required string |
| `#[api_stability("status")]` | API stability status | Required string |
| `#[deadlock_risk("details?")]` | Concurrency issues | Optional string |
| `#[benchmark_candidate("reason?")]` | Needs benchmarking | Optional string |

## Scanner Commands

```sh
# List all macro instances
code-status-scanner list

# Generate summary report
code-status-scanner summary

# Search for specific macros
code-status-scanner search untested,security_sensitive

# Scan specific directory
code-status-scanner -p src/core/ list

# Filter by file pattern (regex)
code-status-scanner -m "model.*\.rs" list

# Exclude files (regex)
code-status-scanner -e "test" list
```

## Local Testing

```sh
# Build all crates
cargo build --workspace

# Run tests
cargo test --workspace

# Try the example
cargo run --example demo_project --features advanced-features

# Test the scanner on the examples
./target/debug/code-status-scanner -p crates/code-status-scanner/examples/ list
./target/debug/code-status-scanner -p crates/code-status-scanner/examples/ summary
```

## Documentation

- [code-status-macros details](crates/code-status-macros/README.md)
- [code-status-scanner details](crates/code-status-scanner/README.md)
- [Example code](crates/code-status-scanner/examples)

## License

MIT or Apache 2.0 