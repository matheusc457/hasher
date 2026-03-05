use blake2::{Blake2b512, Blake2s256, Digest as Blake2Digest};
use crc32fast::Hasher as Crc32Hasher;
use hex::encode;
use md5::{Digest as Md5Digest, Md5};
use ripemd::{Digest as RipemdDigest, Ripemd128, Ripemd160, Ripemd256};
use sha2::{Digest as Sha2Digest, Sha224, Sha256, Sha384, Sha512};
use sha3::{Digest as Sha3Digest, Sha3_256, Sha3_384, Sha3_512};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Algorithm {
    Md5,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Sha3_256,
    Sha3_384,
    Sha3_512,
    Blake2b,
    Blake2s,
    Blake3,
    Ripemd128,
    Ripemd160,
    Ripemd256,
    Crc32,
}

impl Algorithm {
    pub fn all() -> Vec<Algorithm> {
        vec![
            Algorithm::Md5,
            Algorithm::Sha224,
            Algorithm::Sha256,
            Algorithm::Sha384,
            Algorithm::Sha512,
            Algorithm::Sha3_256,
            Algorithm::Sha3_384,
            Algorithm::Sha3_512,
            Algorithm::Blake2b,
            Algorithm::Blake2s,
            Algorithm::Blake3,
            Algorithm::Ripemd128,
            Algorithm::Ripemd160,
            Algorithm::Ripemd256,
            Algorithm::Crc32,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Algorithm::Md5 => "MD5",
            Algorithm::Sha224 => "SHA224",
            Algorithm::Sha256 => "SHA256",
            Algorithm::Sha384 => "SHA384",
            Algorithm::Sha512 => "SHA512",
            Algorithm::Sha3_256 => "SHA3-256",
            Algorithm::Sha3_384 => "SHA3-384",
            Algorithm::Sha3_512 => "SHA3-512",
            Algorithm::Blake2b => "Blake2b",
            Algorithm::Blake2s => "Blake2s",
            Algorithm::Blake3 => "Blake3",
            Algorithm::Ripemd128 => "RIPEMD128",
            Algorithm::Ripemd160 => "RIPEMD160",
            Algorithm::Ripemd256 => "RIPEMD256",
            Algorithm::Crc32 => "CRC32",
        }
    }

    pub fn from_str(s: &str) -> Option<Algorithm> {
        match s.to_lowercase().as_str() {
            "md5" => Some(Algorithm::Md5),
            "sha224" => Some(Algorithm::Sha224),
            "sha256" => Some(Algorithm::Sha256),
            "sha384" => Some(Algorithm::Sha384),
            "sha512" => Some(Algorithm::Sha512),
            "sha3-256" => Some(Algorithm::Sha3_256),
            "sha3-384" => Some(Algorithm::Sha3_384),
            "sha3-512" => Some(Algorithm::Sha3_512),
            "blake2b" => Some(Algorithm::Blake2b),
            "blake2s" => Some(Algorithm::Blake2s),
            "blake3" => Some(Algorithm::Blake3),
            "ripemd128" => Some(Algorithm::Ripemd128),
            "ripemd160" => Some(Algorithm::Ripemd160),
            "ripemd256" => Some(Algorithm::Ripemd256),
            "crc32" => Some(Algorithm::Crc32),
            _ => None,
        }
    }

    pub fn compute(&self, data: &[u8]) -> String {
        match self {
            Algorithm::Md5 => encode(Md5::digest(data)),
            Algorithm::Sha224 => encode(Sha224::digest(data)),
            Algorithm::Sha256 => encode(Sha256::digest(data)),
            Algorithm::Sha384 => encode(Sha384::digest(data)),
            Algorithm::Sha512 => encode(Sha512::digest(data)),
            Algorithm::Sha3_256 => encode(Sha3_256::digest(data)),
            Algorithm::Sha3_384 => encode(Sha3_384::digest(data)),
            Algorithm::Sha3_512 => encode(Sha3_512::digest(data)),
            Algorithm::Blake2b => encode(Blake2b512::digest(data)),
            Algorithm::Blake2s => encode(Blake2s256::digest(data)),
            Algorithm::Blake3 => encode(blake3::hash(data).as_bytes()),
            Algorithm::Ripemd128 => encode(Ripemd128::digest(data)),
            Algorithm::Ripemd160 => encode(Ripemd160::digest(data)),
            Algorithm::Ripemd256 => encode(Ripemd256::digest(data)),
            Algorithm::Crc32 => {
                let mut h = Crc32Hasher::new();
                h.update(data);
                format!("{:08x}", h.finalize())
            }
        }
    }
}
