use std::fs::File;

use super::{
//    get_value_string,
//    get_value,
    read_dict,
    read_int,
};

#[derive(Default, Debug)]
pub struct Camera {
    name: String,
    dist: f32,
    rot: [[f32; 4]; 4],
    offset: i32,
    ortho: bool,
}

impl Camera {
    pub fn new(stream: &File) -> Self {
        let length = read_int(stream);
        let _dict = read_dict(stream, length);

        // TODO: Research on how to parse byte array to float, float array, bool, etc
//        let name = get_value_string(&dict, "name");
//        let dist = get_value_f32(&dict, "dist");
//        let rot = get_value(&dict, "rot");
//        let offset = get_value_i32(&dict, "ofs");
//        let ortho = get_value_bool(&dict, "ortho");
//
//        Camera {
//            name,
//            dist,
//            rot,
//            offset,
//            ortho,
//        }

        Default::default()
    }
}