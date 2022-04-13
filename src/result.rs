use reqwest::blocking;
use std::sync::MutexGuard;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct Result<'a> {
  pub data: MutexGuard<'a, Vec<u8>>,
}

impl<'a> Result<'a> {
  pub fn new(data: MutexGuard<'a, Vec<u8>>) -> Self {
    Self { data }
  }

  pub fn to_file(
    &self, path: &str, 
    buffer: Option<&String>
  ) -> io::Result<()> {
    let url = buffer.as_deref().unwrap();
    let img_bytes = blocking::get(url)
      .unwrap()
      .bytes()
      .unwrap();
    fs::write(path, img_bytes)?;
    
    Ok(())
  }

  pub fn to_buffer(&self) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(self.data.len());
    for byte in self.data.iter() {
      buffer.push(*byte);
    }
      
    buffer
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use lazy_static::lazy_static;
  use std::sync::Mutex;
  use std::path::Path;

  lazy_static! {
    static ref TMP_PATH: &'static str = "./tmp_image.jpg";
    static ref TEST_BUFFER: Mutex<Vec<u8>> = Mutex::new(Vec::new());
  }
  
  #[test]
  fn test_to_buffer() {
    let buffer = fs::read(Path::new(*TMP_PATH)).unwrap();
    for byte in buffer.iter() {
      TEST_BUFFER.lock().unwrap().push(*byte);
    }
    let result = Result::new(TEST_BUFFER.lock().unwrap());
    let new = result.to_buffer();

    assert_eq!(new, buffer);
  }
}
