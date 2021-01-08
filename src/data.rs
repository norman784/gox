use std::str;

use crate::{get_value, read, read_dict, read_int, Block, Bounded, Camera, Layer};

#[derive(Debug)]
pub enum Data {
  Blocks(Block),
  Camera(Camera),
  Layers(Vec<Layer>, Bounded),
}

impl Data {
  pub fn parse(bytes: &mut Vec<u8>) -> Vec<Data> {
    let mut result = vec![];
    let mut layers = vec![];

    loop {
      if bytes.is_empty() {
        break;
      }

      let type_bytes = read(bytes, 4);
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
            let data = Block::new(bytes);
            result.push(Data::Blocks(data));
          }
          "CAMR" => {
            let camera = Camera::new(bytes);
            result.push(Data::Camera(camera));
          }
          "IMG " => {
            let length = read_int(bytes);
            let dict = read_dict(bytes, length);
            let bounds = Bounded::from_bytes(get_value(&dict, "box")).unwrap();
            result.push(Data::Layers(vec![], bounds));
          }
          "LAYR" => {
            let layer = Layer::new(bytes);
            layers.push(layer);
          }
          //"PREV" => {
          //    let length = read_int(bytes);
          //    let _data = read(bytes, length);
          //    let _crc: i32 = read_int(bytes);
          //    result.push(Chunk::Preview);
          //}
          _ => {
            let length = read_int(bytes);
            let _data = read(bytes, length);
          }
        }

        let _crc = read_int(bytes);
      }
    }

    for (i, item) in result.iter().enumerate() {
      if let Data::Layers(_, bounds) = item {
        for layer in layers.iter_mut() {
          if layer.bounds.is_none() {
            layer.bounds = Some(*bounds);
          }
        }

        result[i] = Data::Layers(layers, *bounds);
        break;
      }
    }

    result
  }
}
