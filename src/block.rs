use std::fmt;

use png::{ColorType, Decoder};

use crate::{read, read_int, Memory};

const BLOCK_SIZE: usize = 16;
const BLOCK_SIZE_POT: usize = BLOCK_SIZE * BLOCK_SIZE;
const BLOCK_SIZE_CUBED: usize = BLOCK_SIZE * BLOCK_SIZE * BLOCK_SIZE;

pub struct Block {
  pub colors: Vec<[u8; 4]>,
}

impl Block {
  pub fn new(bytes: &mut Vec<u8>) -> Self {
    let length = read_int(bytes);
    let buffer = read(bytes, length);
    let memory = Memory { buffer };
    let decoder = Decoder::new(memory);
    let mut colors = vec![[0; 4]; BLOCK_SIZE_CUBED];

    match decoder.read_info() {
      Ok((info, mut reader)) => {
        let mut buf = vec![0; info.buffer_size()];

        reader.next_frame(&mut buf).unwrap();

        let data = match info.color_type {
          ColorType::RGB => {
            let mut vec = Vec::with_capacity(buf.len() + buf.len() / 3);
            for rgb in buf.chunks(3) {
              vec.extend([rgb[0], rgb[1], rgb[2], 255].iter())
            }
            vec
          }
          ColorType::RGBA => buf,
          ColorType::Grayscale => {
            let mut vec = Vec::with_capacity(buf.len() * 3);
            for g in buf {
              vec.extend([g, g, g, 255].iter())
            }
            vec
          }
          ColorType::GrayscaleAlpha => {
            let mut vec = Vec::with_capacity(buf.len() * 3);
            for ga in buf.chunks(2) {
              let g = ga[0];
              let a = ga[1];
              vec.extend([g, g, g, a].iter())
            }
            vec
          }
          _ => unreachable!("uncovered color type"),
        };

        for (i, color) in data.chunks(4).enumerate() {
          colors[i] = [color[0], color[1], color[2], color[3]];
        }
      }
      Err(error) => println!("PNG error: {}", error),
    }

    Block { colors }
  }

  pub fn set_pixel(&mut self, x: usize, y: usize, z: usize, value: [u8; 4]) {
    self.colors[Self::index(x, y, z)] = value;
  }

  pub fn get_pixel(&self, x: usize, y: usize, z: usize) -> [u8; 4] {
    self.colors[Self::index(x, y, z)]
  }

  pub fn get_pixel_f32(&self, x: usize, y: usize, z: usize) -> [f32; 4] {
    let color = self.get_pixel(x, y, z);
    [
      color[0] as f32 / 255.0,
      color[1] as f32 / 255.0,
      color[2] as f32 / 255.0,
      color[3] as f32 / 255.0,
    ]
  }

  pub fn is_empty(&self, x: usize, y: usize, z: usize) -> bool {
    self.get_pixel(x, y, z)[3] == 0
  }

  fn index(x: usize, y: usize, z: usize) -> usize {
    x + y * BLOCK_SIZE + z * BLOCK_SIZE_POT
  }
}

impl fmt::Debug for Block {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Block Data ()")
  }
}
