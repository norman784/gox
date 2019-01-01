use std::{
    fs::File,
    str,
};

use crate::{
    Bounded,
    Camera,
    Layer,
    get_value,
    read,
    read_dict,
    read_int,
};

#[derive(Debug)]
pub enum Chunk {
    Block(Vec<u8>),
    Camera(Camera),
    Image(Vec<Layer>, Bounded),
}

impl Chunk {
    pub fn parse(stream: &File) -> Vec<Chunk> {
        let mut result = vec![];
        let mut layers = vec![];

        loop {
            let type_bytes = read(stream, 4);
            let mut zero_count = 0;

            for byte in type_bytes.iter() {
                if byte == &0 {
                    zero_count += 1;
                }
            }

            if zero_count == 4 {
                break;
            }

            if let Ok(type_str) = str::from_utf8(&type_bytes) {
                match type_str {
                    "BL16" => {
                        let length = read_int(stream);
                        let data = read(stream, length);
                        let _crc: i32 = read_int(stream);
                        result.push(Chunk::Block(data));
                    },
                    "CAMR" => {
                        let camera = Camera::new(stream);
                        let _crc: i32 = read_int(stream);
                        result.push(Chunk::Camera(camera));
                    },
                    "IMG " => {
                        let length = read_int(stream);
                        let dict = read_dict(stream, length);
                        let bounds = Bounded::from_bytes(get_value(&dict,  "box")).unwrap();
                        let _crc: i32 = read_int(stream);
                        result.push(Chunk::Image(vec![], bounds));
                    },
                    "LAYR" => {
                        let layer = Layer::new(stream);
                        let _crc: i32 = read_int(stream);
                        layers.push(layer);
                    },
//                    "PREV" => {
//                        let length = read_int(stream);
//                        let _data = read(stream, length);
//                        let _crc: i32 = read_int(stream);
//                        result.push(Chunk::Preview);
//                    },
                    _ => {
                        let length = read_int(stream);
                        let _data = read(stream, length);
                        let _crc: i32 = read_int(stream);
                    },
                }
            }
        }

        for (i, item) in result.iter().enumerate() {
            match item {
                Chunk::Image(_, bounds) => {
                    for layer in layers.iter_mut() {
                        if  layer.bounds.is_none() {
                            layer.bounds = Some(bounds.clone());
                        }
                    }

                    result[i] = Chunk::Image(layers, bounds.clone());
                    break;
                }
                _ => {}
            }
        }

        result
    }
}