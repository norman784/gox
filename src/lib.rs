use std::fs::File;

mod block;
mod camera;
mod chunk;
mod layer;
mod utils;

pub use self::{
    block::Block,
    camera::Camera,
    chunk::Chunk,
    chunk::ChunkData,
    layer::Layer,
    layer::Shape,
    utils::get_value,
    utils::get_value_int,
    utils::get_value_string,
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

#[derive(Eq, PartialEq)]
pub enum Only {
    Block,
    Camera,
    Image,
    Layer,
    Preview,
}

#[derive(Debug)]
pub struct Gox {
    pub version: i32,
    pub data: Vec<Chunk>,
}

impl Gox {
    pub fn new(stream: &File, only: Vec<Only>) -> Self {
//        let stream = &(File::open(file).unwrap());
        let _magic = read(stream, 4);
        let version = read_int(stream);
        let mut chunks = vec![];

        loop {
            if let Some(chunk) = Chunk::new(stream) {
                match chunk.data {
                    ChunkData::Block(_) => {
                        if only.contains(&Only::Block) {
                            chunks.push(chunk)
                        }
                    },
                    ChunkData::Camera(_) => {
                        if only.contains(&Only::Camera) {
                            chunks.push(chunk)
                        }
                    },
                    ChunkData::Image(_) => {
                        if only.contains(&Only::Image) {
                            chunks.push(chunk)
                        }
                    },
                    ChunkData::Layer(_) => {
                        if only.contains(&Only::Layer) {
                            chunks.push(chunk)
                        }
                    },
                    ChunkData::Preview => {
                        if only.contains(&Only::Preview) {
                            chunks.push(chunk)
                        }
                    },
                    _ => {},
                }
            } else {
                break;
            }
        }

        Gox { version, data: chunks }
    }
}