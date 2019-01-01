use std::fs::File;

use crate::read_int;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub index: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Block {
    pub fn new(stream: &File) -> Self {
        let index = read_int(stream);
        let x = read_int(stream);
        let y = read_int(stream);
        let z = read_int(stream);
        let _zero: i32 = read_int(stream);

        Block { index, x, y, z }
    }
}