# Tinify API client for Rust

[![Build Status](https://github.com/Danieroner/tinify-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Danieroner/tinify-rs/actions)
[![crates.io](https://img.shields.io/crates/v/tinify-rs.svg)](https://crates.io/crates/tinify-rs)

Rust client for the Tinify API, used for [TinyPNG](https://tinypng.com) and [TinyJPG](https://tinyjpg.com). Tinify compresses your images intelligently. Read more at [https://tinify.com](https://tinify.com).

## Documentation

[Go to the documentation for the HTTP client](https://tinypng.com/developers/reference).

## Status

In currently development.

There are still features of TinyPNG to implement.

To look at all the features of Tinify API: [Documentation](https://tinypng.com/developers/reference).

## Getting Started

Install the API client with Cargo. Add this to `Cargo.toml`:

```toml
[dependencies]
tinify-rs = "1.0.1"
```
## Usage

- About key

  Get an API key from  https://tinypng.com/developers

- Compress from a file
```rust
use tinify::{Tinify, TinifyException};

fn main() -> Result<(), TinifyException> {
  let key = "tinify api key";
  let optimized = Tinify::new()
    .set_key(key)
    .get_client()?
    .from_file("./unoptimized.png")?
    .to_file("./optimized.png");
 
  Ok(())
}
```

- Compress from an url file
```rust
use tinify::{Tinify, TinifyException};

fn main() -> Result<(), TinifyException> {
  let key = "tinify api key";
  let optimized = Tinify::new()
    .set_key(key)
    .get_client()?
    .from_url("https://tinypng.com/images/panda-happy.png")?
    .to_file("./optimized.png");
 
  Ok(())
}
```

- Compress from a file buffer
```rust
use tinify::{Tinify, TinifyException};
use std::fs;

fn main() -> Result<(), TinifyException> {
  let key = "tinify api key";
  let bytes = fs::read("./unoptimized.png").unwrap();
  let buffer = Tinify::new()
    .set_key(key)
    .get_client()?
    .from_buffer(&bytes)?
    .to_buffer();
  
  let save = fs::write("./optimized.png", buffer).unwrap();
   
  Ok(())
}
```

## Running tests

Create a .env file with a TiniPNG KEY

```
cargo test
```

## Contribution

All contributions will be welcomed. Feel free to open any issues or pull requests.
