use std::convert::TryInto;

use crate::{get_box_size, parse_float};

#[derive(Debug, Default, Copy, Clone)]
pub struct Bounded {
  pub x: i32,
  pub y: i32,
  pub z: i32,
  pub width: usize,
  pub height: usize,
  pub depth: usize,
}

impl Bounded {
  pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
    if !bytes.is_empty() {
      let mut matrix = [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
      ];

      for i in 0..4 {
        for j in 0..4 {
          matrix[i][j] = parse_float(&bytes[(i * 4)..(i * 4 + 4)].try_into().unwrap());
        }
      }

      let (width, height, depth) = get_box_size(&matrix);

      Some(Bounded {
        x: 0,
        y: 0,
        z: 0,
        width,
        height,
        depth,
      })
    } else {
      None
    }
  }
}
