pub use crate::client::Client;
pub use crate::source::Source;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  static ref OPTIONS: Mutex<HashMap<&'static str, String>> = {
    let map = Mutex::new(HashMap::new());
    map
  };
}

/// Set a tinify key
///
/// # Examples
///
/// ```
/// use tinify_rs::tinify;
/// 
/// fn main() {
///   let key = "tinify api key";
///   tinify::set_key(key);
/// }
/// ```
pub fn set_key(new_key: &str) {
  let key = new_key.to_owned();
  OPTIONS.lock().unwrap().insert("key", key);
}

pub fn get_client() -> Client {
  let contains_key = OPTIONS
    .lock()
    .unwrap()
    .contains_key("key");

  if !contains_key {
    panic!("Provide an API key with tinify::set_key(key)");
  }

  let key = OPTIONS
    .lock()
    .unwrap()
    .get("key")
    .unwrap()
    .clone();

  let client = Client::new(key);
  
  client
}

/// Choose a file to compress
///
/// # Examples
///
/// ```
/// use tinify_rs::tinify;
/// 
/// fn main() {
///   tinify::set_key("tinify api key");
/// 
///   let source = tinify::from_file("./unoptimized.png");
///   let compress = source.to_file("./optimized.png");
/// }
/// ```
pub fn from_file(path: &str) -> Source {
  let source = Source::new(None)
    .from_file(path);
  
  source
}

/// Choose a buffer to compress
///
/// # Examples
///
/// ```
/// use tinify_rs::tinify;
/// use std::fs;
/// 
/// fn main() {
///   tinify::set_key("tinify api key");
/// 
///   let bytes = fs::read("./unoptimized.png").unwrap();
///   let buffer = tinify::from_buffer(&bytes).to_buffer();
///   let save = fs::write("./optimized.png", buffer);
/// }
/// ```
pub fn from_buffer(buffer: &Vec<u8>) -> Source {
  let source = Source::new(None)
    .from_buffer(buffer.to_vec());
  
  source
}

/// Choose an url file to compress
///
/// # Examples
///
/// ```
/// use tinify_rs::tinify;
/// 
/// fn main() {
///   tinify::set_key("tinify api key");
/// 
///   let source = tinify::from_url("https://tinypng.com/images/panda-happy.png");
///   let compress = source.to_file("./optimized.png");
/// }
/// ```
pub fn from_url(url: &str) -> Source {
  let source = Source::new(None)
    .from_url(url);

  source
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::mock::MockClient;
  use std::path::Path;
  use std::fs;

  lazy_static! {
    static ref MOCK_CLIENT: MockClient = MockClient::new();
    static ref TMP_PATH: &'static Path = Path::new("./tmp_image.jpg");
  }

  #[test]
  #[should_panic(
    expected="Provide an API key with tinify::set_key(key)"
  )]
  fn test_not_set_key() {
    let contains_key = OPTIONS
      .lock()
      .unwrap()
      .contains_key("key");

    if contains_key {
      OPTIONS
        .lock()
        .unwrap()
        .remove("key")
        .unwrap();
    }

    let source = from_file("./tmp_image.jpg");
    let _compress = source.to_file("./optimized.png");
  }

  #[test]
  fn test_set_key_into_hash_map() {
    set_key(MOCK_CLIENT.key.as_str());
    OPTIONS
      .lock()
      .unwrap()
      .insert("key", MOCK_CLIENT.key.to_owned());

    let test_key = OPTIONS
      .lock()
      .unwrap()
      .get("key")
      .unwrap()
      .clone();

    assert_eq!(test_key, MOCK_CLIENT.key.to_owned());
  }

  #[test]
  fn test_get_one_client() {
    set_key(MOCK_CLIENT.key.as_str());
    let client = get_client();
    let expected = Client {
      key: MOCK_CLIENT.key.to_owned(),
    };
    
    assert_eq!(client, expected);
  }
  
  #[test]
  fn test_from_buffer_get_source() {
    set_key(MOCK_CLIENT.key.as_str());
    let buffer = fs::read(*TMP_PATH).unwrap();
    let source = from_buffer(&buffer);
    let cloned_url = source.url.clone();
    let expected = Source {
      url: cloned_url,
    };

    assert_eq!(source, expected);
  }
}
