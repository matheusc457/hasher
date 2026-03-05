use crate::algorithms::Algorithm;
use rayon::prelude::*;
use std::fs;
use std::path::Path;

pub struct HashResult {
    pub algorithm: String,
    pub hash: String,
}

pub fn hash_file(path: &Path, algorithms: Vec<Algorithm>) -> Result<Vec<HashResult>, String> {
    let data = fs::read(path).map_err(|e| format!("Failed to read file: {}", e))?;

    let results = algorithms
        .par_iter()
        .map(|algo| HashResult {
            algorithm: algo.name().to_string(),
            hash: algo.compute(&data),
        })
        .collect();

    Ok(results)
}
