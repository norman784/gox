use std::{
    collections::HashMap,
    fs::File,
    io::prelude::*,
    str,
};

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