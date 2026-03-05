use crate::hasher::HashResult;

pub fn print_results(filename: &str, results: &[HashResult]) {
    println!("\n{}\n", filename);
    for result in results {
        println!("  {}", result.algorithm);
        println!("  {}", result.hash);
        println!("\n----------------------------------------\n");
    }
}

pub fn print_algorithms(algorithms: &[&str]) {
    println!("\nSupported algorithms:\n");
    for algo in algorithms {
        println!("  {}", algo);
    }
    println!();
}

pub fn print_verify_ok(algo: &str) {
    println!("\n✔ {} — integrity confirmed\n", algo);
}

pub fn print_verify_fail(algo: &str, expected: &str, found: &str) {
    println!("\n✗ {} — hash mismatch", algo);
    println!("  Expected:  {}", expected);
    println!("  Found:     {}\n", found);
}
