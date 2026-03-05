# Hasher

Universal file integrity tool — generate and verify hashes across all major algorithms.

## Installation

### From source

```bash
git clone https://github.com/SEU_USUARIO/hasher
cd hasher
cargo build --release
```

The binary will be available at `target/release/hasher`.

You can move it to your PATH:

```bash
cp target/release/hasher ~/.local/bin/
```

## Usage

### Generate hashes for a file

```bash
hasher generate <file>
```

### Generate hash with a specific algorithm

```bash
hasher generate <file> --algo sha256
```

### Save output to a JSON file

```bash
hasher generate <file> --output result.json
```

### Generate a manifest for a folder

```bash
hasher generate <folder> --output manifest.json
```

### Verify a file against a known hash

```bash
hasher verify <file> --hash <hash> --algo <algorithm>
```

### Verify a folder against a manifest

```bash
hasher verify <folder> --manifest manifest.json
```

### List all supported algorithms

```bash
hasher algorithms
```

## Supported Algorithms

| Algorithm  | Output Size |
|------------|-------------|
| MD5        | 128 bits    |
| SHA224     | 224 bits    |
| SHA256     | 256 bits    |
| SHA384     | 384 bits    |
| SHA512     | 512 bits    |
| SHA3-256   | 256 bits    |
| SHA3-384   | 384 bits    |
| SHA3-512   | 512 bits    |
| Blake2b    | 512 bits    |
| Blake2s    | 256 bits    |
| Blake3     | 256 bits    |
| RIPEMD128  | 128 bits    |
| RIPEMD160  | 160 bits    |
| RIPEMD256  | 256 bits    |
| CRC32      | 32 bits     |

## Building

Requires Rust 1.70 or later.

```bash
cargo build --release
```

## License

This project is licensed under the GNU General Public License v3.0 — see the [LICENSE](LICENSE) file for details.

