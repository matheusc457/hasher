use crate::algorithms::Algorithm;
use crate::hasher::hash_file;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub files: HashMap<String, String>,
    pub algorithm: String,
}

pub fn generate_manifest(folder: &Path, algo: Algorithm, output: &Path) -> Result<(), String> {
    let mut files = HashMap::new();

    visit_dir(folder, folder, &algo, &mut files)?;

    let manifest = Manifest {
        files,
        algorithm: algo.name().to_string(),
    };

    let json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;

    fs::write(output, json).map_err(|e| format!("Failed to write manifest: {}", e))?;

    println!("\nManifest saved to {}\n", output.display());

    Ok(())
}

fn visit_dir(
    base: &Path,
    dir: &Path,
    algo: &Algorithm,
    files: &mut HashMap<String, String>,
) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            visit_dir(base, &path, algo, files)?;
        } else {
            let results = hash_file(&path, vec![algo.clone()])?;
            let relative = path
                .strip_prefix(base)
                .map_err(|e| format!("Failed to strip prefix: {}", e))?
                .to_string_lossy()
                .to_string();
            files.insert(relative, results[0].hash.clone());
        }
    }

    Ok(())
}

pub fn verify_manifest(folder: &Path, manifest_path: &Path) -> Result<(), String> {
    let json =
        fs::read_to_string(manifest_path).map_err(|e| format!("Failed to read manifest: {}", e))?;

    let manifest: Manifest =
        serde_json::from_str(&json).map_err(|e| format!("Failed to parse manifest: {}", e))?;

    let algo = Algorithm::from_str(&manifest.algorithm.to_lowercase())
        .ok_or_else(|| format!("Unknown algorithm: {}", manifest.algorithm))?;

    let mut ok = 0;
    let mut failed = 0;

    println!("\n{}\n", folder.display());

    for (relative, expected_hash) in &manifest.files {
        let path = folder.join(relative);

        if !path.exists() {
            println!("  ✗ {} — file not found", relative);
            failed += 1;
            continue;
        }

        let results = hash_file(&path, vec![algo.clone()])?;
        let found_hash = &results[0].hash;

        if found_hash == expected_hash {
            println!("  ✔ {} — integrity confirmed", relative);
            ok += 1;
        } else {
            println!("  ✗ {} — file modified", relative);
            failed += 1;
        }
    }

    println!("\n----------------------------------------");
    println!("  {} confirmed  {} failed\n", ok, failed);

    Ok(())
}
