#![forbid(overflowing_literals)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
//#![deny(missing_docs)]
#![deny(intra_doc_link_resolution_failure)]
#![deny(path_statements)]
#![deny(trivial_bounds)]
#![deny(type_alias_bounds)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(while_true)]
#![deny(unused)]
#![deny(bad_style)]
#![deny(future_incompatible)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![allow(unused_unsafe)]

use std::fs::File;

mod block;
mod bounded;
mod camera;
mod chunk;
mod layer;
mod utils;

pub use self::{
    block::Block,
    bounded::Bounded,
    camera::Camera,
    chunk::Chunk,
    layer::Layer,
    layer::Shape,
    utils::get_box_size,
    utils::get_value,
    utils::get_value_int,
    utils::get_value_string,
    utils::parse_float,
    utils::read,
    utils::read_dict,
    utils::read_u8,
    utils::read_int,
    utils::read_str,
};

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
    Block,
    Camera,
    Image,
//    Preview,
}

#[derive(Debug)]
pub struct Gox {
    pub version: i32,
    pub data: Vec<Chunk>,
}

impl Gox {
    pub fn new(stream: &File, only: Vec<Only>) -> Self {
        let _magic = read(stream, 4);
        let version = read_int(stream);
        let data = Chunk::parse(stream);
        let mut chunks= vec![];

        if only.len() == 0 {
            chunks = data;
        } else {
            for chunk in data {
                match chunk {
                    Chunk::Block(_) => {
                        if only.contains(&Only::Block) {
                            chunks.push(chunk);
                        }
                    }
                    Chunk::Camera(_) => {
                        if only.contains(&Only::Camera) {
                            chunks.push(chunk);
                        }
                    }
                    Chunk::Image(_, _) => {
                        if only.contains(&Only::Image) {
                            chunks.push(chunk);
                        }
                    }
//                Chunk::Preview(_) => {
//                    if only.contains(&Only::Preview) {
//                        chunks.push(chunk);
//                    }
//                }
                }
            }
        }

        Gox { version, data: chunks }
    }
}