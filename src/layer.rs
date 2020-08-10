use std::io::Read;

use crate::{
    Chunk,
    Bounded,
    Image,
    get_value,
    get_value_int,
    get_value_string,
    read_dict,
    read_int,
};

#[derive(Debug, Copy, Clone)]
pub enum Shape {
    Cube = 1,
    Cylinder = 2,
    Sphere = 0,
}

#[derive(Debug)]
pub struct Layer {
    pub base_id: i32,
    pub blocks: Vec<Chunk>,
    pub id: i32,
    pub image_path: String,
    pub material: Image,
    pub name: String,
    pub shape: Shape,
    pub transform: Vec<u8>,
    pub bounds: Option<Bounded>
}

impl Layer {
    pub fn new(stream: &mut dyn Read) -> Self {
        let mut blocks = vec![];
        let length = read_int(stream);
        let block_count = read_int(stream);
        let block_length = block_count * 20 + 4;

        for _ in 0..block_count {
            blocks.push(Chunk::new(stream));
        }

        let dict = {
            if block_length < length {
                read_dict(stream, length - block_length)
            } else {
                Default::default()
            }
        };

        let base_id = get_value_int(&dict, "base_id");
        let id = get_value_int(&dict, "id");
        let image_path = get_value_string(&dict, "img-path");
        let material = Image { buffer: get_value(&dict, "mat") };
        let name = get_value_string(&dict, "name");
        let shape = get_value_int(&dict, "shape");
        let transform = get_value(&dict, "transform");
        let bounds = Bounded::from_bytes(get_value(&dict, "box"));

        Layer {
            base_id,
            blocks,
            id,
            image_path,
            material,
            name,
            shape: {
                match shape {
                    1 => Shape::Cube,
                    2 => Shape::Cylinder,
                    _ => Shape::Sphere,
                }
            },
            transform,
            bounds,
        }
    }
}