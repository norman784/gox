use crate::{read_dict, read_int};

#[derive(Default, Debug)]
pub struct Camera {
  name: String,
  dist: f32,
  rot: [[f32; 4]; 4],
  offset: i32,
  ortho: bool,
}

impl Camera {
  pub fn new(bytes: &mut Vec<u8>) -> Self {
    let length = read_int(bytes);
    let _dict = read_dict(bytes, length);

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
