use tinify::error::TinifyError;
use tinify::resize::Method;
use tinify::resize::Resize;
use tinify::sync::Tinify;
use std::path::Path;

fn main() -> Result<(), TinifyError> {
  let key = "api key";
  let resize = Resize {
    method: Method::Fit,
    width: Some(150),
    height: Some(100),
  };
  let output = Path::new("./optimized.jpg");
  let tinify = Tinify::new().set_key(key);
  let optimized = tinify
    .get_client()?
    .from_file("./unoptimized.jpg")?
    .resize(resize)?
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
