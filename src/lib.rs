use std::io::Read;

mod block;
mod bounded;
mod camera;
mod chunk;
mod data;
mod image;
mod layer;
mod memory;
mod utils;

pub use crate::{
  block::Block,
  bounded::Bounded,
  camera::Camera,
  chunk::Chunk,
  data::Data,
  image::Image,
  layer::{Layer, Shape},
  memory::Memory,
};

use crate::utils::*;

/*
 * File format, version 2:
 *
 * This is inspired by the png format, where the file consists of a list of
 * chunks with different types.
 *
 *  4 bytes magic string        : "GOX "
 *  4 bytes version             : 2
 *  List of chunks:
 *      4 bytes: type
 *      4 bytes: data length
 *      n bytes: data
 *      4 bytes: CRC
 *
 *  The layer can end with a DICT:
 *      for each entry:
 *          4 byte : key size (0 = end of dict)
 *          n bytes: key
 *          4 bytes: value size
 *          n bytes: value
 *
 *  chunks types:
 *
 *  IMG : a dict of info:
 *      - box: the image gox.
 *
 *  PREV: a png image for preview.
 *
 *  BL16: a 16^3 block saved as a 64x64 png image.
 *
 *  LAYR: a layer:
 *      4 bytes: number of blocks.
 *      for each block:
 *          4 bytes: block index
 *          4 bytes: x
 *          4 bytes: y
 *          4 bytes: z
 *          4 bytes: 0
 *      [DICT]
 *
 *  CAMR: a camera:
 *      [DICT] containing the following entries:
 *          name: string
 *          dist: float
 *          rot: quaternion
 *          ofs: offset
 *          ortho: bool
 *
 */

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Only {
  Blocks,
  Camera,
  Layers,
  //    Preview,
}

#[derive(Debug)]
pub struct Gox {
  pub version: i32,
  pub data: Vec<Data>,
}

impl Gox {
  pub fn new(stream: &mut dyn Read, only: Vec<Only>) -> Self {
    let bytes = stream
      .bytes()
      .map(|r| match r {
        Ok(r) => Some(r),
        _ => None,
      })
      .filter(|r| r.is_some())
      .map(|r| r.unwrap())
      .collect::<Vec<u8>>();
    Self::from_bytes(bytes, only)
  }

  pub fn from_bytes(bytes: Vec<u8>, only: Vec<Only>) -> Self {
    let mut bytes = bytes;
    let _magic = bytes.drain(..4).as_slice();
    let version = read_int(&mut bytes);
    let data = Data::parse(&mut bytes);
    let mut chunks = vec![];

    if only.is_empty() {
      chunks = data;
    } else {
      for chunk in data {
        match chunk {
          Data::Blocks(_) => {
            if only.contains(&Only::Blocks) {
              chunks.push(chunk);
            }
          }
          Data::Camera(_) => {
            if only.contains(&Only::Camera) {
              chunks.push(chunk);
            }
          }
          Data::Layers(_, _) => {
            if only.contains(&Only::Layers) {
              chunks.push(chunk);
            }
          }
        }
      }
    }

    Gox {
      version,
      data: chunks,
    }
  }
}

impl From<Vec<u8>> for Gox {
  fn from(bytes: Vec<u8>) -> Self {
    Self::from_bytes(bytes, vec![Only::Layers, Only::Blocks])
  }
}
