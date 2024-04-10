use tinify::error::TinifyError;
use tinify::async_bin::Tinify;
use tinify::resize::Resize;
use tinify::resize::Method;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), TinifyError> {
  let key = "api key";
  let output = Path::new("./optimized.jpg");
  let resize = Resize {
    method: Method::Fit,
    width: Some(150),
    height: Some(100),
  };
  let tinify = Tinify::new().set_key(key);
  let optimized = tinify
    .get_async_client()?
    .from_file("./unoptimized.jpg").await?
    .resize(resize)?
    .to_file(output).await;

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
