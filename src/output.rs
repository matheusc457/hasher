use crate::hasher::HashResult;

pub fn print_results(filename: &str, results: &[HashResult]) {
    println!("\n{}", filename);
    let last = results.len() - 1;
    for (i, result) in results.iter().enumerate() {
        let prefix = if i == last { "└──" } else { "├──" };
        println!("{}  {:<12} {}", prefix, result.algorithm, result.hash);
    }
    println!();
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

