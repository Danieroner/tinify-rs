# Tinify API client for Rust 🦀

<p align="center">
  <a href="https://github.com/Danieroner/tinify-rs/actions">
    <img alt="CI Status" src="https://github.com/Danieroner/tinify-rs/actions/workflows/ci.yml/badge.svg" />
  </a>
  <a href="https://crates.io/crates/tinify-rs">
    <img alt="Crate Version" src="https://img.shields.io/crates/v/tinify-rs.svg" />
  </a>
</p>

Tinify API Client for the Rust Programming Language, used for [TinyPNG](https://tinypng.com) and [TinyJPG](https://tinyjpg.com). Tinify compresses your images intelligently. Read more at [https://tinify.com](https://tinify.com).

## Documentation

[Go to the documentation for the HTTP client](https://tinypng.com/developers/reference).

## Status

In currently development.

There are still features of TinyPNG to implement.

To look at all the features of Tinify API: [Documentation](https://tinypng.com/developers/reference).

## Roadmap

 * [x] Compressing images
 * [x] Resizing images
 * [x] Converting images
 * [ ] Preserving metadata
 * [ ] Saving to Amazon S3
 * [ ] Saving to Google Cloud Storage
 * [x] Implement an async non-blocking Client


## Getting Started

Install the API client with Cargo. Add this to `Cargo.toml`:

```toml
[dependencies]
tinify-rs = "1.4.2"
```

Using async client

```toml
[dependencies]
tinify-rs = { version = "1.4.2", features = ["async"] }
```

## Usage

- About key

  Get an API key from  https://tinypng.com/developers

- Compress from a file
```rust
use tinify::error::TinifyError;
use tinify::sync::Tinify;
use std::path::Path;

fn main() -> Result<(), TinifyError> {
  let key = "api key";
  let output = Path::new("./optimized.jpg");
  let tinify = Tinify::new().set_key(key);
  let optimized = tinify
    .get_client()?
    .from_file("./unoptimized.jpg")?
    .to_file(output);

  if let Err(error) = optimized {
    match error {
      TinifyError::ClientError { ref upstream } => {
        println!("Error: {} message: {}", upstream.error, upstream.message);
      }
      _ => println!("{:?}", error),
    }
  }

  Ok(())
}

```

- Compress from a file async
```rust
use tinify::error::TinifyError;
use tinify::async_bin::Tinify;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), TinifyError> {
  let key = "api key";
  let output = Path::new("./optimized.jpg");
  let tinify = Tinify::new().set_key(key);
  let compressed = tinify
    .get_async_client()?
    .from_file("./unoptimized.jpg").await?
    .to_file(output).await;

  if let Err(error) = compressed {
    match error {
      TinifyError::ClientError { ref upstream } => {
        println!("Error: {} message: {}", upstream.error, upstream.message);
      }
      _ => println!("{:?}", error),
    }
  }

  Ok(())
}

```

## Running tests

Create a .env file with a TiniPNG KEY

```
cargo test --features async
```

## Contribution

All contributions will be welcomed. Feel free to open any issues or pull requests.
