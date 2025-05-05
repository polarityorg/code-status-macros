# Code Status Scanner

A command-line tool to scan Rust codebases for [code-status-macros](../code-status-macros) usage and generate reports.

## Installation

```bash
# Install from crates.io
cargo install code-status-scanner

# Or build from source
git clone https://github.com/polarityorg/code-status-macros.git
cd code-status-macros
cargo install --path crates/code-status-scanner
```

## Features

- Scan entire codebases for code status macro usage
- Generate summary reports of technical debt and development issues
- Search for specific types of markers
- Filter files using regex patterns
- Color-coded output for better readability

## Usage

### Basic Commands

```bash
# Scan the current directory and list all markers
code-status-scanner

# Scan a specific directory
code-status-scanner --path /path/to/project

# Generate a summary report
code-status-scanner summary

# Search for specific macros
code-status-scanner search "untested,needs_review"
```

### Filtering Options

```bash
# Only scan files matching a pattern
code-status-scanner --pattern "src/.*\.rs"

# Exclude files matching a pattern
code-status-scanner --exclude "tests/.*\.rs"

# Combine patterns and commands
code-status-scanner --pattern "src/.*\.rs" --exclude ".*_test\.rs" summary
```

## Output Examples

### List Format

```
Found 42 code status macro instances:

src/main.rs:45 #[untested]
    fn process_data(input: &str) -> Result<Data, Error> {

src/auth.rs:128 #[security_sensitive]
    fn validate_token(token: &str) -> bool {

src/parser.rs:256 #[needs("better error handling")]
    fn parse_complex_input(data: &[u8]) -> Vec<Token> {
```

### Summary Format

```
== Macro Usage Summary ==
Total macro instances: 42

By macro type:
  untested                 : 12
  needs                    : 9
  security_sensitive       : 7
  needs_review             : 5
  unsafe_usage             : 4
  temporary                : 3
  complexity               : 2

Top 5 files by macro usage:
  src/auth.rs                                         : 8
  src/parser.rs                                       : 7
  src/main.rs                                         : 6
  src/models/user.rs                                  : 5
  src/api/endpoints.rs                                : 4
```

## Use Cases

- Track technical debt across a codebase
- Identify areas that need review before a release
- Generate reports for team discussions
- Plan refactoring efforts
- Monitor the use of unsafe code or potential panic points

## Development

This tool is part of the [code-status-macros](https://github.com/polarityorg/code-status-macros) collection of development utilities.

To contribute or report issues, please visit the GitHub repository. 