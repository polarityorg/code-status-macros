//! Code Status Macros
//!
//! A collection of procedural macros for annotating code with development status markers
//! and tracking technical debt. These macros provide a standardized way for teams to mark
//! and track various development concerns in code.
//!
//! # Available Macros
//!
//! ## Code Quality Markers
//!
//! - [`untested`] - Marks functions that haven't been properly tested
//! - [`includes_unwrap`] - Indicates code containing unwrap() calls that could panic
//! - [`needs`] - Indicates a specific need (e.g., refactoring, optimization)
//! - [`perf_critical`] - Marks code that needs performance optimization
//! - [`security_sensitive`] - Marks code with known security implications
//! - [`unsafe_usage`] - Marks code that uses unsafe blocks and needs careful auditing
//! - [`no_clippy`] - Marks code where certain clippy lints are deliberately suppressed
//! - [`complexity`] - Indicates high algorithm or cognitive complexity issues
//! - [`allocation_heavy`] - Flags functions that perform significant heap allocations
//! - [`panic_path`] - Highlights code paths that might panic under specific conditions
//!
//! ## Review & Future Work Markers
//!
//! - [`needs_review`] - Indicates code that requires special review before release
//! - [`temporary`] - Marks code as temporary or intended to be replaced
//! - [`assumptions`] - Indicates code with non-obvious assumptions
//! - [`revisit_in`] - Marks code that may need revisiting in a future version
//! - [`dependency_sensitive`] - Marks code that's sensitive to changes in dependencies
//! - [`platform_specific`] - Indicates code with behavior tied to specific platforms
//! - [`feature_gated`] - Marks code dependent on specific feature flags
//! - [`api_stability`] - Indicates parts of the API that may change
//! - [`deadlock_risk`] - Marks code with potential concurrency/deadlock issues
//! - [`benchmark_candidate`] - Flags code that should be benchmarked and optimized

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, LitStr};

/// A marker attribute to indicate that a function is untested.
/// This attribute does not modify the function it annotates.
#[proc_macro_attribute]
pub fn untested(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree item (like a function).
    let item_ast = parse_macro_input!(item as syn::Item);

    // Use quote to reconstruct the token stream for the item.
    // This effectively returns the original function unchanged.
    TokenStream::from(quote! { #item_ast })
}

/// A marker attribute to indicate a specific need for an item (e.g., function).
/// Accepts a string literal describing the need, like `#[needs("refactoring")]`.
/// Can be applied multiple times to the same item.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn needs(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute argument (the string literal).
    // We expect a single string literal, e.g., #[needs("some reason")]
    // We parse it to ensure it's a valid string literal, but don't use the value.
    let _reason = parse_macro_input!(attr as LitStr);

    // Parse the input tokens into a syntax tree item (like a function).
    let item_ast = parse_macro_input!(item as Item);

    // Use quote to reconstruct the token stream for the item.
    // This effectively returns the original item unchanged.
    TokenStream::from(quote! { #item_ast })
}

/// A marker attribute to indicate that a function contains `unwrap()` calls.
/// This helps identify potential panic points in code.
/// This attribute does not modify the function it annotates.
#[proc_macro_attribute]
pub fn includes_unwrap(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree item (like a function).
    let item_ast = parse_macro_input!(item as syn::Item);

    // Use quote to reconstruct the token stream for the item.
    // This effectively returns the original function unchanged.
    TokenStream::from(quote! { #item_ast })
}

/// Mark code that needs performance optimization.
/// This helps identify areas that could be bottlenecks.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn perf_critical(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as syn::Item);
    TokenStream::from(quote! { #item_ast })
}

/// Mark code with known security implications.
/// This helps identify areas that might need security auditing.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn security_sensitive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as syn::Item);
    TokenStream::from(quote! { #item_ast })
}

/// Indicate code that requires special review before release.
/// This helps identify areas that need careful review by team members.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn needs_review(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as syn::Item);
    TokenStream::from(quote! { #item_ast })
}

/// Mark code as temporary or intended to be replaced.
/// This helps identify code that should not be considered permanent.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn temporary(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as syn::Item);
    TokenStream::from(quote! { #item_ast })
}

/// Indicate that code has non-obvious assumptions.
/// Accepts a string literal describing the assumptions, like `#[assumptions("assumes sorted input")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn assumptions(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _description = parse_macro_input!(attr as LitStr);
    let item_ast = parse_macro_input!(item as Item);
    TokenStream::from(quote! { #item_ast })
}

/// Mark code that may need revisiting in a future version.
/// Accepts a string literal describing when to revisit, like `#[revisit_in("v2.0")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn revisit_in(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _version = parse_macro_input!(attr as LitStr);
    let item_ast = parse_macro_input!(item as Item);
    TokenStream::from(quote! { #item_ast })
}

/// Mark code that's sensitive to changes in dependencies.
/// This helps identify code that might break when dependencies are updated.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn dependency_sensitive(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_ast = parse_macro_input!(item as syn::Item);
    TokenStream::from(quote! { #item_ast })
}

/// Mark code that uses unsafe blocks and needs careful auditing.
/// Optionally accepts a string literal describing the reason for unsafe usage,
/// like `#[unsafe_usage("raw pointer arithmetic for performance")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn unsafe_usage(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _reason = if !attr.is_empty() {
        Some(parse_macro_input!(attr as LitStr))
    } else {
        None
    };
    let item_ast = parse_macro_input!(item as syn::Item);
    TokenStream::from(quote! { #item_ast })
}

/// Mark code where certain clippy lints are deliberately suppressed.
/// Accepts a string literal describing which lints and why,
/// like `#[no_clippy("too_many_arguments: this API needs to be flexible")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn no_clippy(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _description = parse_macro_input!(attr as LitStr);
    let item_ast = parse_macro_input!(item as Item);
    TokenStream::from(quote! { #item_ast })
}

/// Indicate code with behavior tied to specific platforms.
/// Accepts a string literal describing the platform dependencies,
/// like `#[platform_specific("windows")]` or `#[platform_specific("linux, macos")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn platform_specific(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _platforms = parse_macro_input!(attr as LitStr);
    let item_ast = parse_macro_input!(item as Item);
    TokenStream::from(quote! { #item_ast })
}

/// Mark code dependent on specific feature flags.
/// Accepts a string literal describing the feature dependency,
/// like `#[feature_gated("async")]` or `#[feature_gated("extended-api")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn feature_gated(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _feature = parse_macro_input!(attr as LitStr);
    let item_ast = parse_macro_input!(item as Item);
    TokenStream::from(quote! { #item_ast })
}

/// Indicate algorithm complexity or cognitive complexity issues.
/// Accepts a string literal describing the complexity,
/// like `#[complexity("O(n²)")]` or `#[complexity("high: many nested conditions")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn complexity(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _description = parse_macro_input!(attr as LitStr);
    let item_ast = parse_macro_input!(item as Item);
    TokenStream::from(quote! { #item_ast })
}

/// Flag functions that perform significant heap allocations.
/// Optionally accepts a string literal with additional details,
/// like `#[allocation_heavy("allocates vectors for each input item")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn allocation_heavy(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _details = if !attr.is_empty() {
        Some(parse_macro_input!(attr as LitStr))
    } else {
        None
    };
    let item_ast = parse_macro_input!(item as syn::Item);
    TokenStream::from(quote! { #item_ast })
}

/// Highlight code paths that might panic under specific conditions.
/// Accepts a string literal describing the potential panic scenarios,
/// like `#[panic_path("fails if input is empty")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn panic_path(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _scenario = parse_macro_input!(attr as LitStr);
    let item_ast = parse_macro_input!(item as Item);
    TokenStream::from(quote! { #item_ast })
}

/// Indicate parts of the API that may change.
/// Accepts a string literal describing the stability level,
/// like `#[api_stability("unstable")]`, `#[api_stability("experimental")]`,
/// or `#[api_stability("deprecated: use new_function() instead")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn api_stability(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _stability = parse_macro_input!(attr as LitStr);
    let item_ast = parse_macro_input!(item as Item);
    TokenStream::from(quote! { #item_ast })
}

/// Mark code with potential concurrency/deadlock issues.
/// Optionally accepts a string literal detailing the risk,
/// like `#[deadlock_risk("acquires multiple locks")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn deadlock_risk(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _details = if !attr.is_empty() {
        Some(parse_macro_input!(attr as LitStr))
    } else {
        None
    };
    let item_ast = parse_macro_input!(item as syn::Item);
    TokenStream::from(quote! { #item_ast })
}

/// Flag code that should be benchmarked and optimized.
/// Optionally accepts a string literal with benchmarking notes,
/// like `#[benchmark_candidate("bottleneck in processing pipeline")]`.
/// This attribute does not modify the item it annotates.
#[proc_macro_attribute]
pub fn benchmark_candidate(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _notes = if !attr.is_empty() {
        Some(parse_macro_input!(attr as LitStr))
    } else {
        None
    };
    let item_ast = parse_macro_input!(item as syn::Item);
    TokenStream::from(quote! { #item_ast })
}
