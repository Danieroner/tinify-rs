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
