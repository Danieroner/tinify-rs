#![allow(dead_code)]

use crate::error::TinifyResult;
use crate::client::{Method, Client};
use std::path::Path;

#[macro_export]
macro_rules! create_file {
  () => {
    use image;
    use image::RgbImage;
    use num_complex;
    use std::path::Path;

    let img_x = 400;
    let img_y = 400;
    let scale_x = 3.0 / img_x as f32;
    let scale_y = 3.0 / img_y as f32;
    let mut img_buf: RgbImage = image::ImageBuffer::new(img_x, img_y);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
      let r = (1.3 * x as f32) as u8;
      let b = (1.3 * y as f32) as u8;
      *pixel = image::Rgb([r, 0, b]);
    }

    for x in 0..img_x {
      for y in 0..img_y {
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;
        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);
        let mut i = 0;

        while i < 255 && z.norm() <= 2.0 {
          z = z * z + c;
          i += 1;
        }

        let pixel = img_buf.get_pixel_mut(x, y);
        let image::Rgb(data) = *pixel;
        *pixel = image::Rgb([data[0], i as u8, data[2]]);
      }
    }
  
    let tmp = Path::new("./tmp_test_image.png");
    img_buf.save(tmp).unwrap();
  }
}

pub struct MockClient<'a> {
  pub key: &'a str,
}

impl<'a> MockClient<'a> {
  pub fn new() -> Self {
    Self {
      key: "yjb7YwqyRZwzkGtCfDt6qmXs3QRQTJz3",
    }
  }

  pub fn request(
    &self, 
    method: Method, 
    path: &Path, 
    buffer: Option<&Vec<u8>>
  ) -> TinifyResult {
    let client = Client {
      key: self.key.to_owned(),
    };
    client.request(method, path, buffer)
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::Path;
  
  #[test]
  fn test_tmp_file_was_created() {
    create_file!();
    let path = Path::new("./tmp_test_image.png");
    
    assert!(path.exists());
  }

  #[test]
  fn test_tmp_file_was_deleted() {
    let path = Path::new("./tmp_test_image.png");
    if path.exists() {
      fs::remove_file(path).unwrap();
    }

    assert!(!path.exists());
  }
}
