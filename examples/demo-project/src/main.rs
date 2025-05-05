//! Demo Project
//!
//! This is an example file that demonstrates the use of all code status macros
//! from the code-status-macros crate. It's designed to show how you might use
//! these macros in a real project to track technical debt and development concerns.

use code_status_macros::*;

// =====================================
// Code Quality Markers Examples
// =====================================

#[untested]
fn calculate_statistics(data: &[f64]) -> (f64, f64, f64) {
    // This function calculates mean, median, and standard deviation
    let sum: f64 = data.iter().sum();
    let mean = sum / data.len() as f64;

    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = if data.len() % 2 == 0 {
        (sorted[data.len() / 2 - 1] + sorted[data.len() / 2]) / 2.0
    } else {
        sorted[data.len() / 2]
    };

    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;
    let std_dev = variance.sqrt();

    (mean, median, std_dev)
}

#[includes_unwrap]
fn parse_config(config_str: &str) -> Vec<String> {
    // This function unwraps results without proper error handling
    config_str
        .lines()
        .map(|line| line.split('=').collect::<Vec<&str>>())
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts[1].trim().to_string())
        .collect()
}

#[needs("proper error handling")]
#[needs("input validation")]
fn process_user_input(input: &str) -> Result<(), String> {
    // This function needs multiple improvements
    if input.is_empty() {
        return Err("Empty input".into());
    }

    // Process the input
    println!("Processing: {}", input);

    Ok(())
}

#[perf_critical]
fn heavy_computation(size: usize) -> Vec<f64> {
    // This function is performance-critical
    let mut result = Vec::with_capacity(size);

    for i in 0..size {
        // Expensive calculation
        let mut value = 0.0;
        for j in 0..1000 {
            value += (i * j) as f64 / 1000.0;
        }
        result.push(value);
    }

    result
}

#[security_sensitive]
fn validate_user_credentials(username: &str, password: &str) -> bool {
    // Security-sensitive code for authentication
    // This is a simplified example - never implement auth like this!
    username == "admin" && password == "password123"
}

#[unsafe_usage("raw pointer manipulation for performance")]
fn direct_memory_access(data: &mut [u8]) {
    // Using unsafe code for direct memory manipulation
    let len = data.len();

    unsafe {
        let ptr = data.as_mut_ptr();
        for i in 0..len {
            *ptr.add(i) = (*ptr.add(i)).wrapping_add(1);
        }
    }
}

#[no_clippy("too_many_arguments: this API needs to match external system")]
fn external_system_api(
    param1: i32,
    param2: i32,
    param3: i32,
    param4: i32,
    param5: i32,
    param6: i32,
    param7: i32,
    param8: i32,
) -> i32 {
    // This function has too many arguments but matches an external API
    param1 + param2 + param3 + param4 + param5 + param6 + param7 + param8
}

#[complexity("O(n²) - nested loops have quadratic time complexity")]
fn matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    // Matrix multiplication with O(n²) complexity
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();

    let mut result = vec![vec![0.0; cols_b]; rows_a];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    result
}

#[allocation_heavy("creates a new vector for each input element")]
fn transform_all_elements<T: Clone, U>(items: &[T], transform: fn(&T) -> U) -> Vec<Vec<U>> {
    // Creates many allocations that could potentially be optimized
    items
        .iter()
        .map(|item| {
            let mut result = Vec::new();
            result.push(transform(item));
            result
        })
        .collect()
}

#[panic_path("divides by zero when count is zero")]
fn calculate_average(sum: f64, count: usize) -> f64 {
    // This function will panic if count is zero
    sum / count as f64
}

// =====================================
// Review & Future Work Markers Examples
// =====================================

#[needs_review]
fn sensitive_data_processing(user_data: &[String]) -> Vec<String> {
    // This function processes sensitive user data and needs review
    user_data
        .iter()
        .map(|item| {
            // Some processing of sensitive data
            format!("PROCESSED: {}", item)
        })
        .collect()
}

#[temporary]
fn legacy_data_conversion(data: &str) -> String {
    // Temporary function that should be replaced with a better solution
    data.replace(',', "|").replace(';', ",")
}

#[assumptions("input array is already sorted")]
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    // Binary search that assumes the input array is already sorted
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if arr[mid] == target {
            return Some(mid);
        }

        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

#[revisit_in("v2.0")]
fn current_implementation() -> String {
    // Current implementation that should be revisited in version 2.0
    "Current implementation".into()
}

#[dependency_sensitive]
fn uses_external_library() -> String {
    // This function relies on behaviors of external dependencies
    format!("Using external libraries: {}", env!("CARGO_PKG_NAME"))
}

#[platform_specific("unix")]
fn unix_only_function() -> &'static str {
    // This function is meant for Unix-like systems only
    #[cfg(unix)]
    {
        return "Running on Unix";
    }

    #[cfg(not(unix))]
    {
        panic!("This function is Unix-only");
    }
}

#[feature_gated("advanced-features")]
fn advanced_feature() -> &'static str {
    // This function should only be available when the "advanced-features" feature is enabled
    #[cfg(feature = "advanced-features")]
    {
        return "Advanced feature enabled";
    }

    #[cfg(not(feature = "advanced-features"))]
    {
        return "Advanced feature not enabled";
    }
}

#[api_stability("experimental")]
fn experimental_api() -> &'static str {
    // This API is experimental and may change in future versions
    "Experimental API"
}

#[deadlock_risk("acquires multiple locks in nested calls")]
fn nested_lock_acquisition() {
    // This function might deadlock due to acquiring multiple locks
    // In a real implementation, this would involve actual mutexes
    println!("Acquiring lock 1");
    // Lock 1 acquired
    println!("Acquiring lock 2");
    // Lock 2 acquired
    println!("Both locks acquired");
}

#[benchmark_candidate("critical path in rendering pipeline")]
fn render_frame(scene: &[u8]) -> Vec<u8> {
    // This function is a good candidate for benchmarking and optimization
    let mut frame = Vec::with_capacity(scene.len());

    for &pixel in scene {
        // Some expensive rendering logic
        let processed = (pixel as f64 * 1.5).min(255.0) as u8;
        frame.push(processed);
    }

    frame
}

// =====================================
// Example Struct with Multiple Markers
// =====================================

#[security_sensitive]
#[needs_review]
struct User {
    id: u64,
    username: String,
    password_hash: String,
}

impl User {
    #[untested]
    #[needs("proper password hashing")]
    fn new(id: u64, username: String, password: String) -> Self {
        User {
            id,
            username,
            password_hash: format!("hash:{}", password), // NOT secure, just an example
        }
    }

    #[security_sensitive]
    fn verify_password(&self, password: &str) -> bool {
        // This is NOT secure password verification, just for example purposes
        self.password_hash == format!("hash:{}", password)
    }
}

// =====================================
// Example Trait with Markers
// =====================================

#[needs("comprehensive documentation")]
#[api_stability("unstable")]
trait DataProcessor {
    #[untested]
    fn process(&self, data: &[u8]) -> Vec<u8>;

    #[needs_review]
    fn validate(&self, data: &[u8]) -> bool;
}

// =====================================
// Main Function
// =====================================

fn main() {
    println!("This is an example file demonstrating all code status macros");
    println!("Run the code-status-scanner on this file to see how it detects these macros");

    // Demonstrate accessing feature-gated functionality
    println!("{}", advanced_feature());
}
