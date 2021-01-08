use crate::read_int;

#[derive(Debug, Copy, Clone)]
pub struct Chunk {
  pub block_index: usize,
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

impl Chunk {
  pub fn new(bytes: &mut Vec<u8>) -> Self {
    let block_index = read_int(bytes) as usize;
    let x = read_int(bytes);
    let y = read_int(bytes);
    let z = read_int(bytes);
    let _zero: i32 = read_int(bytes);

    Chunk {
      block_index,
      x,
      y,
      z,
    }
  }
}
