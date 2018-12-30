use std::fs::File;

use super::{
    Block,
    get_value,
    get_value_int,
    get_value_string,
    read_dict,
    read_int,
};

#[derive(Debug)]
pub enum Shape {
    Cube = 1,
    Cylinder = 2,
    Sphere = 0,
}

#[derive(Debug)]
pub struct Layer {
    pub base_id: i32,
    pub blocks: Vec<Block>,
    pub id: i32,
    pub image_path: String,
    pub material: Vec<u8>,
    pub name: String,
    pub shape: Shape,
    pub transform: Vec<u8>,
}

impl Layer {
    pub fn new(stream: &File) -> Self {
        let mut blocks = vec![];
        let length = read_int(stream);
        let block_count = read_int(stream);
        let block_length = block_count * 20 + 4;

        for _ in 0..block_count {
            blocks.push(Block::new(stream));
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
        let material = get_value(&dict, "mat");
        let name = get_value_string(&dict, "name");
        let shape = get_value_int(&dict, "shape");
        let transform = get_value(&dict, "transform");

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
        }
    }
}