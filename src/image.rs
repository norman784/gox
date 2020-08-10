use std::{
    fmt,
    io::Read,
};

use crate::{
    read,
    read_int,
};

pub struct Image {
   pub buffer: Vec<u8>,
}

impl Image {
    pub fn new(stream: &mut dyn Read) -> Self {
        let length = read_int(stream);
        let buffer = read(stream, length);

        Image { buffer }
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Image ()")
    }
}