//! Code Status Scanner
//!
//! A CLI tool to scan Rust codebases for code-status-macros usage and generate reports.

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use colored::Colorize;
use regex::Regex;
use walkdir::WalkDir;

/// List of all macros from code-status-macros
const MACRO_NAMES: &[&str] = &[
    // Code Quality Markers
    "untested",
    "includes_unwrap",
    "needs",
    "perf_critical",
    "security_sensitive",
    "unsafe_usage",
    "no_clippy",
    "complexity",
    "allocation_heavy",
    "panic_path",
    // Review & Future Work Markers
    "needs_review",
    "temporary",
    "assumptions",
    "revisit_in",
    "dependency_sensitive",
    "platform_specific",
    "feature_gated",
    "api_stability",
    "deadlock_risk",
    "benchmark_candidate",
];

/// Common directories to exclude for better performance
const DEFAULT_EXCLUDE_DIRS: &[&str] = &[
    "target/",
    "node_modules/",
    ".git/",
    ".idea/",
    ".vscode/",
    "dist/",
    "build/",
];

/// CLI arguments
#[derive(Parser)]
#[command(name = "code-status-scanner")]
#[command(author, version, about = "Scans Rust code for code-status-macros usage", long_about = None)]
struct Cli {
    /// Path to the directory to scan (defaults to current directory)
    #[arg(short, long, default_value = ".")]
    path: String,

    /// Only scan files matching this pattern (regex)
    #[arg(short = 'm', long)]
    pattern: Option<String>,

    /// Exclude files matching this pattern (regex)
    #[arg(short, long)]
    exclude: Option<String>,

    /// Maximum directory depth to scan (default: no limit)
    #[arg(short, long)]
    max_depth: Option<usize>,

    /// Skip default excluded directories (target/, node_modules/, etc.)
    #[arg(short = 'S', long, default_value_t = true)]
    skip_default_dirs: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all macros found in the codebase
    List,
    /// Generate a summary report of macro usage
    Summary,
    /// Search for specific macros
    Search {
        /// Macros to search for (comma-separated)
        #[arg(required = true)]
        macros: String,
    },
}

/// Represents a found macro in the code
#[derive(Debug)]
struct MacroInstance {
    path: PathBuf,
    line: usize,
    macro_name: String,
    argument: Option<String>,
    context: String,
}

/// Pre-compile all regexes for better performance
fn create_macro_regexes() -> Vec<(String, Regex)> {
    MACRO_NAMES
        .iter()
        .map(|&name| {
            let pattern = format!("#\\[{}(.*?)\\]", name);
            (
                name.to_string(),
                Regex::new(&pattern).expect("Failed to compile regex pattern"),
            )
        })
        .collect()
}

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);

    // Compile regex patterns if provided
    let include_pattern = cli
        .pattern
        .map(|p| Regex::new(&p).expect("Invalid include pattern"));
    let exclude_pattern = cli
        .exclude
        .map(|p| Regex::new(&p).expect("Invalid exclude pattern"));

    // Pre-compile all the regexes we'll need
    let macro_regexes = create_macro_regexes();

    // Find all macros in the codebase
    let instances = scan_directory(
        path,
        &include_pattern,
        &exclude_pattern,
        &macro_regexes,
        cli.max_depth,
        cli.skip_default_dirs,
    );

    if instances.is_empty() {
        println!(
            "{}",
            "No code status macros found in the codebase.".yellow()
        );
        return;
    }

    match &cli.command {
        Some(Commands::List) => list_macros(&instances),
        Some(Commands::Summary) => generate_summary(&instances),
        Some(Commands::Search { macros }) => search_macros(&instances, macros),
        None => list_macros(&instances), // Default to list if no subcommand provided
    }
}

/// Scan a directory for code status macros
fn scan_directory(
    path: &Path,
    include_pattern: &Option<Regex>,
    exclude_pattern: &Option<Regex>,
    macro_regexes: &[(String, Regex)],
    max_depth: Option<usize>,
    skip_default_dirs: bool,
) -> Vec<MacroInstance> {
    let mut instances = Vec::new();
    let max_depth = max_depth.unwrap_or(usize::MAX);

    // First collect all eligible files to avoid recursive regex checks
    let walker = WalkDir::new(path)
        .follow_links(true)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(Result::ok);

    let files: Vec<PathBuf> = walker
        .filter(|entry| {
            let path = entry.path();

            // Skip if not a file
            if !path.is_file() {
                return false;
            }

            // Skip if not a Rust file
            if !path.to_string_lossy().ends_with(".rs") {
                return false;
            }

            // Skip default excluded directories if enabled
            if skip_default_dirs {
                let path_str = path.to_string_lossy();
                if DEFAULT_EXCLUDE_DIRS
                    .iter()
                    .any(|&dir| path_str.contains(dir))
                {
                    return false;
                }
            }

            // Apply include/exclude patterns
            let path_str = path.to_string_lossy();
            if let Some(pattern) = include_pattern {
                if !pattern.is_match(&path_str) {
                    return false;
                }
            }

            if let Some(pattern) = exclude_pattern {
                if pattern.is_match(&path_str) {
                    return false;
                }
            }

            true
        })
        .map(|entry| entry.path().to_path_buf())
        .collect();

    // Process each file
    for path in &files {
        if let Ok(content) = fs::read_to_string(path) {
            let mut file_instances = scan_file(path, &content, macro_regexes);
            instances.append(&mut file_instances);
        }
    }

    instances
}

/// Scan a single file for code status macros
fn scan_file(path: &Path, content: &str, macro_regexes: &[(String, Regex)]) -> Vec<MacroInstance> {
    let mut instances = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        // Check for macros in the current line
        for (macro_name, regex) in macro_regexes {
            if let Some(caps) = regex.captures(line) {
                let argument = caps.get(1).map(|m| m.as_str().trim().to_string());

                // Get context (next line after the macro)
                let context = if line_idx + 1 < lines.len() {
                    lines[line_idx + 1].trim().to_string()
                } else {
                    String::new()
                };

                instances.push(MacroInstance {
                    path: path.to_path_buf(),
                    line: line_idx + 1,
                    macro_name: macro_name.clone(),
                    argument,
                    context,
                });
            }
        }
    }

    instances
}

/// List all macros found in the codebase
fn list_macros(instances: &[MacroInstance]) {
    println!(
        "{}",
        format!("Found {} code status macro instances:", instances.len()).green()
    );
    println!();

    for instance in instances {
        let rel_path = instance.path.display();
        let line_info = format!("{}:{}", rel_path, instance.line);
        let arg_display = match &instance.argument {
            Some(arg) if !arg.is_empty() => format!("({})", arg.yellow()),
            _ => "".to_string(),
        };

        println!(
            "{} {}{}",
            line_info.blue(),
            format!("#[{}]", instance.macro_name).green(),
            arg_display
        );
        println!("    {}", instance.context.trim());
        println!();
    }
}

/// Generate a summary report of macro usage
fn generate_summary(instances: &[MacroInstance]) {
    let mut count_by_macro = HashMap::new();
    let mut count_by_file = HashMap::new();

    for instance in instances {
        *count_by_macro
            .entry(instance.macro_name.clone())
            .or_insert(0) += 1;
        *count_by_file.entry(instance.path.clone()).or_insert(0) += 1;
    }

    println!("{}", "== Macro Usage Summary ==".green().bold());
    println!(
        "{}",
        format!("Total macro instances: {}", instances.len()).cyan()
    );
    println!();

    println!("{}", "By macro type:".yellow());
    for (macro_name, count) in count_by_macro.iter() {
        println!("  {:25} : {}", macro_name, count);
    }
    println!();

    println!("{}", "Top 5 files by macro usage:".yellow());
    let mut files: Vec<_> = count_by_file.iter().collect();
    files.sort_by(|a, b| b.1.cmp(a.1));

    for (file, count) in files.iter().take(5) {
        println!("  {:50} : {}", file.display(), count);
    }
}

/// Search for specific macros
fn search_macros(instances: &[MacroInstance], macros_str: &str) {
    let macro_names: Vec<&str> = macros_str.split(',').map(|s| s.trim()).collect();
    let filtered: Vec<_> = instances
        .iter()
        .filter(|i| macro_names.contains(&i.macro_name.as_str()))
        .collect();

    if filtered.is_empty() {
        println!(
            "{}",
            format!("No macros found matching: {}", macros_str).yellow()
        );
        return;
    }

    println!(
        "{}",
        format!("Found {} instances of requested macros:", filtered.len()).green()
    );
    println!();

    for instance in &filtered {
        let rel_path = instance.path.display();
        let line_info = format!("{}:{}", rel_path, instance.line);
        let arg_display = match &instance.argument {
            Some(arg) if !arg.is_empty() => format!("({})", arg.yellow()),
            _ => "".to_string(),
        };

        println!(
            "{} {}{}",
            line_info.blue(),
            format!("#[{}]", instance.macro_name).green(),
            arg_display
        );
        println!("    {}", instance.context.trim());
        println!();
    }
}
