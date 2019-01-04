use std::{
    fmt,
    io::Read,
};

use png::{
    Decoder,
    ColorType,
};

use crate::{
    Memory,
    read,
    read_int,
};

pub struct Block {
    colors: Vec<Vec<Vec<[u8; 4]>>>
}

impl Block {
    pub fn new(stream: &mut dyn Read) -> Self {
        let length = read_int(stream);
        let buffer = read(stream, length);
        let memory = Memory { buffer: buffer.clone() };
        let decoder = Decoder::new(memory);
        let mut result = vec![vec![vec![[0; 4]; 16]; 16]; 16];
        let mut colors = vec![[0; 4];  4096];

        match decoder.read_info() {
            Ok((info, mut reader)) => {
//            println!(
//                "PNG w: {} h: {} bit_depth: {:?} buffer_size: {} color_type: {:?}",
//                info.width,
//                info.height,
//                info.bit_depth,
//                info.buffer_size(),
//                info.color_type
//            );

                let mut buf = vec![0; info.buffer_size()];

                reader.next_frame(&mut buf).unwrap();

                let data = match info.color_type {
                    ColorType::RGB => buf,
                    ColorType::RGBA => buf,
                    ColorType::Grayscale => {
                        let mut vec = Vec::with_capacity(buf.len() * 3);
                        for g in buf {
                            vec.extend([g, g, g].iter().cloned())
                        }
                        vec
                    },
                    ColorType::GrayscaleAlpha => {
                        let mut vec = Vec::with_capacity(buf.len() * 3);
                        for ga in buf.chunks(2) {
                            let g = ga[0];
                            let a = ga[1];
                            vec.extend([g, g, g, a].iter().cloned())
                        }
                        vec
                    },
                    _ => unreachable!("uncovered color type")
                };

                // TODO: Need to think a better way to read the values directly instead of generating another intermediate array
                let mut i = 0;
                for color in data.chunks(4) {
                    colors[i] = [color[0] << 0, color[1] << 0, color[2] << 0, color[3] << 0];
                    i += 1;
                }

                let size = 16;
                let size_cubed = size * size;

                for x in 0..size {
                    for y in 0..size {
                        for z in 0..size {
                            let index = x + y * size + size_cubed * z;
                            result[x][y][z] = colors[index];
                        }
                    }
                }
            },
            Err(error) => println!("PNG error: {}", error),
        }

        Block {
            colors: result
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, z:usize, value: [u8; 4]) {
        self.colors[x][y][z] = value;
    }

    pub fn get_pixel(&self, x: usize, y: usize, z:usize) -> [u8; 4] {
        self.colors[x][y][z]
    }

    pub fn is_empty(&self, x: usize, y: usize, z:usize) -> bool {
        self.get_pixel(x, y, z)[3] == 0
    }
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block Data ()")
    }
}