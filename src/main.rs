mod algorithms;
mod cli;
mod hasher;
mod output;

use clap::Parser;
use cli::{Cli, Commands};
use std::path::Path;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate { file, algo, output } => {
            let path = Path::new(&file);

            if !path.exists() {
                eprintln!("Error: file '{}' not found", file);
                std::process::exit(1);
            }

            let algorithms = match algo {
                Some(ref a) => match algorithms::Algorithm::from_str(a) {
                    Some(alg) => vec![alg],
                    None => {
                        eprintln!("Error: unknown algorithm '{}'", a);
                        std::process::exit(1);
                    }
                },
                None => algorithms::Algorithm::all(),
            };

            match hasher::hash_file(path, algorithms) {
                Ok(results) => {
                    output::print_results(&file, &results);

                    if let Some(out) = output {
                        let json = serde_json::to_string_pretty(
                            &results
                                .iter()
                                .map(|r| {
                                    serde_json::json!({
                                        "algorithm": r.algorithm,
                                        "hash": r.hash,
                                        "file": file,
                                    })
                                })
                                .collect::<Vec<_>>(),
                        )
                        .unwrap();
                        std::fs::write(&out, json).unwrap();
                        println!("Saved to {}", out);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Verify {
            file,
            hash,
            algo,
            manifest: _,
        } => {
            let path = Path::new(&file);

            if !path.exists() {
                eprintln!("Error: file '{}' not found", file);
                std::process::exit(1);
            }

            let (hash_value, algorithm) = match (hash, algo) {
                (Some(h), Some(a)) => (h, a),
                _ => {
                    eprintln!("Error: --hash and --algo are required for verify");
                    std::process::exit(1);
                }
            };

            let alg = match algorithms::Algorithm::from_str(&algorithm) {
                Some(a) => a,
                None => {
                    eprintln!("Error: unknown algorithm '{}'", algorithm);
                    std::process::exit(1);
                }
            };

            match hasher::hash_file(path, vec![alg]) {
                Ok(results) => {
                    let result = &results[0];
                    if result.hash == hash_value {
                        output::print_verify_ok(&result.algorithm);
                    } else {
                        output::print_verify_fail(&result.algorithm, &hash_value, &result.hash);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Algorithms => {
            let names: Vec<&str> = algorithms::Algorithm::all()
                .iter()
                .map(|a| a.name())
                .collect();
            output::print_algorithms(&names);
        }
    }
}
