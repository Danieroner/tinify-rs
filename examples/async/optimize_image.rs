use tinify::error::TinifyError;
use tinify::async_bin::Tinify;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), TinifyError> {
  let key = "api key";
  let output = Path::new("./optimized.jpg");
  let tinify = Tinify::new().set_key(key);
  let optimized = tinify
    .get_async_client()?
    .from_file("./unoptimized.jpg").await?
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
