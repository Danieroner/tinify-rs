use tinify::error::TinifyError;
use tinify::transform::Transform;
use tinify::convert::Convert;
use tinify::convert::Type;
use tinify::sync::Tinify;
use std::path::Path;

fn main() -> Result<(), TinifyError> {
  let key = "api key";
  let convert = Convert {
    r#type: vec![Type::Jpeg],
  };
  let transform = Transform {
    background: "#800020".to_string(),
  };
  let output = Path::new("./optimized.jpg");
  let tinify = Tinify::new().set_key(key);
  let optimized = tinify
    .get_client()?
    .from_url("https://tinypng.com/images/panda-happy.png")?
    .convert(convert)?
    .transform(transform)?
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
