use std::{
    fs::File,
    str,
};

use super::{
    Camera,
    Layer,
    read,
    read_int,
};

#[derive(Debug)]
pub enum ChunkData {
    Block(Vec<u8>),
    Camera(Camera),
    Image(Vec<u8>),
    Layer(Layer),
    Preview,
    Unkown,
}

impl ChunkData {
    pub fn new(stream: &File) -> Option<Self> {
        let type_bytes = read(stream, 4);
        let mut zero_count = 0;

        for byte in type_bytes.iter() {
            if byte == &0 {
                zero_count += 1;
            }
        }

        if zero_count == 4 {
            return None
        }

        if let Ok(type_str) = str::from_utf8(&type_bytes) {
            match type_str {
                "BL16" => {
                    let length = read_int(stream);
                    let data = read(stream, length);
                    let _crc: i32 = read_int(stream);
                    Some(ChunkData::Block(data))
                },
                "CAMR" => {
                    let camera = Camera::new(stream);
                    let _crc: i32 = read_int(stream);
                    Some(ChunkData::Camera(camera))
                },
                "IMG " => {
                    let length = read_int(stream);
                    let data = read(stream, length);
                    let _crc: i32 = read_int(stream);
                    Some(ChunkData::Image(data))
                },
                "LAYR" => {
                    let layer = Layer::new(stream);
                    let _crc: i32 = read_int(stream);
                    Some(ChunkData::Layer(layer))
                },
                "PREV" => {
                    let length = read_int(stream);
                    let _data = read(stream, length);
                    let _crc: i32 = read_int(stream);
                    Some(ChunkData::Preview)
                },
                _ => {
                    let length = read_int(stream);
                    let _data = read(stream, length);
                    Some(ChunkData::Unkown)
                },
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    pub data: ChunkData,
}

impl Chunk {
    pub fn new(stream: &File) -> Option<Self> {
        if let Some(data) = ChunkData::new(stream) {
            Some(Chunk { data })
        } else {
            None
        }
    }
}