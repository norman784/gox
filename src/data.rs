use std::{io::Read, str};

use crate::{get_value, read, read_dict, read_int, Block, Bounded, Camera, Layer};

#[derive(Debug)]
pub enum Data {
    Blocks(Block),
    Camera(Camera),
    Layers(Vec<Layer>, Bounded),
}

impl Data {
    pub fn parse(stream: &mut dyn Read) -> Vec<Data> {
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
                        let data = Block::new(stream);
                        result.push(Data::Blocks(data));
                    }
                    "CAMR" => {
                        let camera = Camera::new(stream);
                        result.push(Data::Camera(camera));
                    }
                    "IMG " => {
                        let length = read_int(stream);
                        let dict = read_dict(stream, length);
                        let bounds = Bounded::from_bytes(get_value(&dict, "box")).unwrap();
                        result.push(Data::Layers(vec![], bounds));
                    }
                    "LAYR" => {
                        let layer = Layer::new(stream);
                        layers.push(layer);
                    }
                    //"PREV" => {
                    //    let length = read_int(stream);
                    //    let _data = read(stream, length);
                    //    let _crc: i32 = read_int(stream);
                    //    result.push(Chunk::Preview);
                    //}
                    _ => {
                        let length = read_int(stream);
                        let _data = read(stream, length);
                    }
                }

                let _crc = read_int(stream);
            }
        }

        for (i, item) in result.iter().enumerate() {
            match item {
                Data::Layers(_, bounds) => {
                    for layer in layers.iter_mut() {
                        if layer.bounds.is_none() {
                            layer.bounds = Some(bounds.clone());
                        }
                    }

                    result[i] = Data::Layers(layers, bounds.clone());
                    break;
                }
                _ => {}
            }
        }

        result
    }
}
