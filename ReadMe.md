# Compressor-Decompressor in Rust

This is a command line application written in Rust that compresses and decompresses files using the GZIP format.

## Installation

Before you can run this project, make sure you have Rust installed on your machine. If you don't, you can install it by following the instructions [here](https://www.rust-lang.org/tools/install).

To install the compressor-decompressor, clone the repository using the following command:

```bash
git clone https://github.com/emmanuelbyte/compressor-decompressor.git
```

## Usage

After installation, you can use the `compressor-decompressor` to compress and decompress files using the compress and decompress commands respectively.

To compress a file, use the following command:

```bash
 cargo run -- compress <input-file> <output-file.gz>
```

This will compress <input-file> and create a new compressed file named <output-file.gz>.

To decompress a file, use the following command:

```bash
cargo run -- decompress <input-file.gz> <output-file>
```

This will decompress <input-file.gz> and create a new decompressed file named <output-file>.

### Contributing

If you'd like to contribute to this project, please feel free to make a pull request. All contributions are welcome!
