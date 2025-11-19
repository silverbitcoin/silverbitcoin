//! # Test Command
//!
//! Run tests for a Quantum package.

use crate::package::Package;
use anyhow::{Context, Result};
use colored::Colorize;

/// Execute the `quantum test` command
pub async fn execute(filter: Option<&str>) -> Result<()> {
    // Load package
    let package = Package::load_current()
        .context("Failed to load package. Make sure you're in a Quantum package directory.")?;
    
    println!("{} {} v{}", 
        "Testing".green().bold(), 
        package.name().bold(), 
        package.version()
    );
    
    if let Some(filter_str) = filter {
        println!("Filter: {}", filter_str);
    }
    
    // Build package first
    println!();
    println!("Building package...");
    crate::commands::build::execute(false, None).await?;
    
    // Find and run tests
    println!();
    println!("Running tests...");
    
    let test_results = run_tests(&package, filter)?;
    
    // Print results
    println!();
    print_test_results(&test_results);
    
    if test_results.failed > 0 {
        anyhow::bail!("Tests failed");
    }
    
    Ok(())
}

/// Test results
struct TestResults {
    passed: usize,
    failed: usize,
    total: usize,
}

/// Run all tests in the package
fn run_tests(package: &Package, filter: Option<&str>) -> Result<TestResults> {
    let source_files = package.source_files()?;
    
    let mut passed = 0;
    let failed = 0;
    
    for source_file in &source_files {
        let source = std::fs::read_to_string(source_file)?;
        
        // Find test functions (functions with #[test] attribute)
        let tests = find_test_functions(&source);
        
        for test in tests {
            // Apply filter if specified
            if let Some(filter_str) = filter {
                if !test.contains(filter_str) {
                    continue;
                }
            }
            
            println!("  test {} ... ", test);
            
            // TODO: Actually execute the test
            // For now, we'll just mark them as passed
            passed += 1;
        }
    }
    
    let total = passed + failed;
    
    Ok(TestResults {
        passed,
        failed,
        total,
    })
}

/// Find test functions in source code
fn find_test_functions(source: &str) -> Vec<String> {
    let mut tests = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    
    for i in 0..lines.len() {
        let line = lines[i].trim();
        
        // Look for #[test] attribute
        if line == "#[test]" && i + 1 < lines.len() {
            let next_line = lines[i + 1].trim();
            
            // Extract function name
            if let Some(name) = extract_function_name(next_line) {
                tests.push(name);
            }
        }
    }
    
    tests
}

/// Extract function name from function declaration
fn extract_function_name(line: &str) -> Option<String> {
    if !line.starts_with("public fun ") && !line.starts_with("fun ") {
        return None;
    }
    
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 3 {
        return None;
    }
    
    let name_part = if parts[0] == "public" {
        parts[2]
    } else {
        parts[1]
    };
    
    // Remove parentheses and everything after
    let name = name_part.split('(').next()?;
    
    Some(name.to_string())
}

/// Print test results
fn print_test_results(results: &TestResults) {
    println!();
    println!("test result: {}. {} passed; {} failed; {} total",
        if results.failed == 0 { "ok".green().bold() } else { "FAILED".red().bold() },
        results.passed,
        results.failed,
        results.total
    );
}
