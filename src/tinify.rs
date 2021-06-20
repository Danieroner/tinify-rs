use crate::client::Client;
use crate::source::Source;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use std::process;

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
/// let key = "tinify api key";
/// tinify::set_key(key);
/// ```
pub fn set_key(new_key: &str) {
  let key = new_key.to_string();
  OPTIONS.lock().unwrap().insert("key", key);
}

pub fn get_client() -> Client {
  let key = OPTIONS.lock().unwrap().get("key").unwrap().clone();
  if key.len() == 0 {
    eprintln!("Provide an API key with tinify::set_key(key)");
    process::exit(1);
  }
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::create_file;
  use crate::client::Client;
  use lazy_static::lazy_static;
  use std::sync::Once;
  use std::path::Path;
  use std::fs;

  lazy_static! {
    static ref INIT: Once = Once::new();
    static ref PRIVATE_KEY: &'static str = "yjb7YwqyRZwzkGtCfDt6qmXs3QRQTJz3";
    static ref TMP_PATH: &'static str = "./tmp_test_image.png";
    static ref CLIENT: Client = Client {
      key: String::from(*PRIVATE_KEY),
    };
  }

  fn initialize() {
    INIT.call_once(|| {
      set_key(*PRIVATE_KEY);
    });
  }

  #[test]
  fn test_set_key_into_hash_map() {
    initialize();
    let test_key = OPTIONS.lock().unwrap().get("key").unwrap().clone();

    assert_eq!(test_key, String::from(*PRIVATE_KEY));
  }

  #[test]
  fn test_get_one_client() {
    initialize();
    let client = get_client();
    let expected = Client {
      key: String::from(*PRIVATE_KEY),
    };
    
    assert_eq!(client, expected);
  }

  #[test]
  fn test_from_file_get_source() {
    initialize();
    let path = Path::new(*TMP_PATH);
    if !path.exists() {
      create_file!();
    }
    let source = from_file(*TMP_PATH);
    let cloned_url = source.url.clone();
    let expected = Source {
      url: cloned_url,
    };
    if path.exists() {
      fs::remove_file(path).unwrap();
    }
    
    assert_eq!(source, expected);
  }
  
  #[test]
  fn test_from_buffer_get_source() {
    initialize();
    let path = Path::new(*TMP_PATH);
    if !path.exists() {
      create_file!();
    }
    let buffer = fs::read(path).unwrap();
    let source = from_buffer(&buffer);
    let cloned_url = source.url.clone();
    let expected = Source {
      url: cloned_url,
    };
    if path.exists() {
      fs::remove_file(path).unwrap();
    }

    assert_eq!(source, expected);
  }
}
