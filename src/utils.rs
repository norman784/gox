use std::{
    collections::HashMap,
    fs::File,
    io::prelude::*,
    str,
};

use byteorder::{ByteOrder, LittleEndian};

// TODO: Need to check if the math is correct
pub fn get_box_size(matrix: &[[f32; 4]; 4]) -> (u32, u32, u32) {
    let mut result = [0.0; 3];
    let mut values = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ];

    for (i, value) in values.iter_mut().enumerate() {
        let vector = mat4_mul_vec4(&matrix, value);
        result[i] = vec3_norm(&vector);
    }

    // FIXME: For some reason the size is half of the supposed value, so for now I just multiply by 2 to get the proper size
    ((result[0] * 2.0) as u32, (result[1] * 2.0) as u32, (result[2] * 2.0) as u32)
}

fn mat4_mul_vec4(matrix: &[[f32; 4]; 4], vector: &[f32; 4]) -> [f32; 4] {
    let mut result = [0.0; 4];

    for i in 0..4 {
        for j in 0..4 {
            result[i] += matrix[i][j] * vector[j];
        }
    }

    result
}

pub fn parse_float(byte_array: &[u8; 4]) -> f32 {
    LittleEndian::read_f32(byte_array)
}

pub fn read(stream: &File, bytes: i32) -> Vec<u8> {
    let mut result = vec![];

    for _ in 0..bytes {
        result.push(read_u8(stream));
    }

    result
}

pub fn read_u8(mut stream: &File) -> u8 {
    let mut buffer = [0; 1];
    let _result = stream.read(&mut buffer);

    (buffer[0] << 0)
}

pub fn read_int(mut stream: &File) -> i32 {
    let mut buffer = [0; 4];
    let _result = stream.read(&mut buffer);

    ((buffer[0] as i32) <<  0) +
        ((buffer[1] as i32) <<  8) +
        ((buffer[2] as i32) << 16) +
        ((buffer[3] as i32) << 24)
}

pub fn read_str(stream: &File, bytes: i32) -> String {
    let result = read(stream, bytes);
    match str::from_utf8(&result) {
        Ok(string) => string.to_string(),
        _ => "".to_string()
    }
}

pub fn read_dict(stream: &File, max_bytes: i32) -> HashMap<String, Vec<u8>> {
    let mut result = HashMap::new();
    let mut bytes_read = 0;

    if max_bytes == 0 {
        return result;
    }

    while let Some((key, value, key_value_length)) = read_dict_key_value(stream) {
        result.insert(key, value);
        bytes_read += key_value_length;

        if bytes_read >= max_bytes {
            break;
        }
    }

    result
}

pub fn read_dict_key_value(stream: &File) -> Option<(String, Vec<u8>, i32)> {
    let key_size = read_int(stream);
    if key_size > 0 {
        let key = read_str(stream, key_size);
        let value_size = read_int(stream);

        let value = read(stream, value_size);
        Some((key.to_string(), value, key_size + value_size + 8))
    } else {
        None
    }
}

pub fn get_value(dict: &HashMap<String, Vec<u8>>, key: &str) -> Vec<u8> {
    if let Some(value) = dict.get(key) {
        value.to_vec()
    } else {
        Default::default()
    }
}

pub fn get_value_string(dict: &HashMap<String, Vec<u8>>, key: &str) -> String {
    let value = get_value(dict, &key);
    if value.len() > 0 {
        str::from_utf8(&value).unwrap().to_string()
    } else {
        "".to_string()
    }
}

pub fn get_value_int(dict: &HashMap<String, Vec<u8>>, key: &str) -> i32 {
    let value = get_value(dict, &key);

    if value.len() > 0 {
        ((value[0] as i32) <<  0) +
            ((value[1] as i32) <<  8) +
            ((value[2] as i32) << 16) +
            ((value[3] as i32) << 24)
    } else {
        0
    }
}

fn vec3_dot(lhs: &[f32; 4], rhs: &[f32; 4]) -> f32 {
    lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

fn vec3_norm(vector: &[f32; 4])  -> f32 {
    let dot = vec3_dot(vector, vector);
    dot.sqrt()
}