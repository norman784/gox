use std::io::Read;

use crate::read_int;

#[derive(Debug, Copy, Clone)]
pub struct Chunk {
    pub block_index: usize,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Chunk {
    pub fn new(stream: &mut dyn Read) -> Self {
        let block_index = read_int(stream) as usize;
        let x = read_int(stream);
        let y = read_int(stream);
        let z = read_int(stream);
        let _zero: i32 = read_int(stream);

        Chunk { block_index, x, y, z }
    }
}