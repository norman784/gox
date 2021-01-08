use std::fmt;

use crate::{read, read_int};

pub struct Image {
  pub buffer: Vec<u8>,
}

impl Image {
  pub fn new(bytes: &mut Vec<u8>) -> Self {
    let length = read_int(bytes);
    let buffer = read(bytes, length);

    Image { buffer }
  }
}

impl fmt::Debug for Image {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Image ()")
  }
}
