# Code Status Macros Demo Project

This is a demonstration project showing how to use all the code status macros in a realistic codebase.

## Running the Example

```sh
# Run without the advanced-features
cargo run -p code-status-demo

# Run with the advanced-features enabled
cargo run -p code-status-demo --features advanced-features
```

## Scanning the Example with Code Status Scanner

To see how the scanner tool can detect and report on the macro usage in this demo:

```sh
# Build the scanner
cargo build -p code-status-scanner

# Run the scanner on this example
./target/debug/code-status-scanner -p examples/demo-project list

# Generate a summary report
./target/debug/code-status-scanner -p examples/demo-project summary
```

## What's Included

This example demonstrates:

- 20 different code status macros
- Usage on functions, structs, traits, and methods
- Macros with and without parameters
- Multiple macros applied to the same item
- Feature-gated code
- Platform-specific code

## Feature Flags

This example has one feature flag:

- `advanced-features` - Enables the "advanced feature" functionality demonstrated in the `advanced_feature()` function

The example works with or without this feature enabled, but running it with both settings demonstrates the feature-dependent behavior. 