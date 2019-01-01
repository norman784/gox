use crate::{
    get_box_size,
    parse_float,
};

#[derive(Debug, Default, Copy, Clone)]
pub struct Bounded {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl Bounded {
    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        if bytes.len() > 0 {
            let mut  matrix = [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ];

            let mut index = 0;
            for i in 0..4 {
                for j in 0..4 {
                    matrix[i][j] = parse_float(&[
                        bytes[index],
                        bytes[index+1],
                        bytes[index+2],
                        bytes[index+3]
                    ]);

                    index += 4;
                }
            }

            let x = 0;
            let y = 0;
            let z = 0;
            let (width, height, depth) = get_box_size(&matrix);

            Some(Bounded { x, y, z, width, height, depth })
        } else {
            None
        }
    }
}