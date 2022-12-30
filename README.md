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
tinify-rs = "1.1.0"
```
## Usage

- About key

  Get an API key from  https://tinypng.com/developers

- Compress from a file
```rust
use tinify::Tinify;
use tinify::TinifyError;

fn main() -> Result<(), TinifyError> {
  let key = "tinify api key";
  let tinify = Tinify::new().set_key(key);
  let client = tinify.get_client()?;
  let _ = client
    .from_file("./unoptimized.jpg")?
    .to_file("./optimized.jpg")?;
    
  Ok(())
}
```

- Compress from an url
```rust
use tinify::Tinify;
use tinify::TinifyError;

fn main() -> Result<(), TinifyError> {
  let key = "tinify api key";
  let tinify = Tinify::new().set_key(key);
  let client = tinify.get_client()?;
  let _ = client
    .from_url("https://tinypng.com/images/panda-happy.png")?
    .to_file("./optimized.png")?;
    
  Ok(())
}
```

- Compress from a buffer
```rust
use tinify::Tinify;
use tinify::TinifyError;
use std::fs;

fn main() -> Result<(), TinifyError> {
  let key = "tinify api key";
  let tinify = Tinify::new().set_key(key);
  let client = tinify.get_client()?;
  let bytes = fs::read("./unoptimized.jpg")?;
  let _ = client
    .from_buffer(&bytes)?
    .to_file("./optimized.jpg")?;
     
  Ok(())
}
```

- Resize a compressed image.
```rust
use tinify::Tinify;
use tinify::Client;
use tinify::TinifyError;
use tinify::resize::Method;
use tinify::resize::Resize;

fn get_client() -> Result<Client, TinifyError> {
  let key = "tinify api key";
  let tinify = Tinify::new();

  tinify
    .set_key(key)
    .get_client()
}

fn main() -> Result<(), TinifyError> {
  let client = get_client()?;
  let _ = client
    .from_file("./unoptimized.jpg")?
    .resize(Resize::new(
      Method::FIT,
      Some(400),
      Some(200)),
    )?
    .to_file("./resized.jpg")?;

  Ok(())
}
```

- Converting images.
```rust
use tinify::Tinify;
use tinify::convert::Type;
use tinify::TinifyError;

fn main() -> Result<(), TinifyError> {
  let _ = Tinify::new()
  .set_key("api key")
  .get_client()?
  .from_file("./tmp_image.jpg")?
  .convert((
      Some(Type::PNG),
      None,
      None,
    ),
    None,
  )?
  .to_file("./converted.png");

  Ok(())
}
```

- Converting images with a transparent background.
```rust
use tinify::Tinify;
use tinify::convert::Color;
use tinify::convert::Type;
use tinify::TinifyError;

fn main() -> Result<(), TinifyError> {
  let _ = Tinify::new()
  .set_key("api key")
  .get_client()?
  .from_url("https://tinypng.com/images/panda-happy.png")?
  .convert((
      Some(Type::JPEG),
      None,
      None,
    ),
    Some(Color("#FF5733")),
  )?
  .to_file("./converted.jpg");

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
