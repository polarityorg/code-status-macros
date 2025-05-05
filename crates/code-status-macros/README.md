# Code Status Macros

A collection of procedural macros for annotating code with development status markers and tracking technical debt.

## Purpose

These macros provide a standardized way for teams to mark and track various development concerns in code. They help create a shared vocabulary for discussing code quality issues and areas that need attention before release.

## Available Macros

### Code Quality Markers

- `#[untested]` - Marks functions that haven't been properly tested
- `#[includes_unwrap]` - Indicates code containing unwrap() calls that could panic
- `#[needs("reason")]` - Indicates a specific need (e.g., refactoring, optimization)
- `#[perf_critical]` - Marks code that needs performance optimization
- `#[security_sensitive]` - Marks code with known security implications
- `#[unsafe_usage("reason")]` - Marks code that uses unsafe blocks and needs careful auditing
- `#[no_clippy("reason")]` - Marks code where certain clippy lints are deliberately suppressed
- `#[complexity("details")]` - Indicates high algorithm or cognitive complexity issues
- `#[allocation_heavy("details")]` - Flags functions that perform significant heap allocations
- `#[panic_path("scenario")]` - Highlights code paths that might panic under specific conditions

### Review & Future Work Markers

- `#[needs_review]` - Indicates code that requires special review before release
- `#[temporary]` - Marks code as temporary or intended to be replaced
- `#[assumptions("detail")]` - Indicates code with non-obvious assumptions
- `#[revisit_in("v2.0")]` - Marks code that may need revisiting in a future version
- `#[dependency_sensitive]` - Marks code that's sensitive to changes in dependencies
- `#[platform_specific("platform")]` - Indicates code with behavior tied to specific platforms
- `#[feature_gated("feature")]` - Marks code dependent on specific feature flags
- `#[api_stability("status")]` - Indicates parts of the API that may change
- `#[deadlock_risk("details")]` - Marks code with potential concurrency/deadlock issues
- `#[benchmark_candidate("reason")]` - Flags code that should be benchmarked and optimized

## Usage

Add the crate to your dependencies:

```toml
[dependencies]
code-status-macros = "0.1.0"
```

Import and use the macros:

```rust
use code_status_macros::*;

#[untested]
#[needs("better error handling")]
fn my_function() {
    // ...
}

#[security_sensitive]
#[needs_review]
fn handle_authentication() {
    // ...
}

#[unsafe_usage("raw pointer arithmetic for performance")]
#[complexity("O(n²)")]
#[benchmark_candidate("bottleneck in processing pipeline")]
fn complex_calculation() {
    // ...
}

#[platform_specific("windows")]
#[feature_gated("extended-api")]
fn windows_specific_feature() {
    // ...
}
```

## Scanner Tool

This crate comes with a companion CLI tool called `code-status-scanner` for finding and reporting on all code status macros in your codebase.

### Installing the Scanner

```bash
cargo install code-status-scanner
```

### Scanner Usage

Basic usage:

```bash
# Scan the current directory for all macros
code-status-scanner

# Scan a specific directory
code-status-scanner --path /path/to/your/project

# Generate a summary report
code-status-scanner summary

# Search for specific macros
code-status-scanner search "untested,needs_review"

# Filter files by regex pattern
code-status-scanner --pattern "src/.*\.rs" --exclude "test/.*\.rs"
```

### Scanner Commands

- `list` (default): Lists all macro instances with their location and context
- `summary`: Generates a summary report of macro usage
- `search`: Searches for specific macros (comma-separated list)

## Note

These macros are designed as lightweight markers. They do not modify the code they annotate and serve purely as standardized documentation. 