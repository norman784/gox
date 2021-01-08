use std::{collections::HashMap, str};

use byteorder::{ByteOrder, LittleEndian};

// TODO: Need to check if the math is correct
pub fn get_box_size(matrix: &[[f32; 4]; 4]) -> (usize, usize, usize) {
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
  (
    (result[0] * 2.0) as usize,
    (result[1] * 2.0) as usize,
    (result[2] * 2.0) as usize,
  )
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
  if !value.is_empty() {
    str::from_utf8(&value).unwrap().to_string()
  } else {
    "".to_string()
  }
}

pub fn get_value_int(dict: &HashMap<String, Vec<u8>>, key: &str) -> i32 {
  let value = get_value(dict, &key);

  if !value.is_empty() {
    (value[0] as i32)
      + ((value[1] as i32) << 8)
      + ((value[2] as i32) << 16)
      + ((value[3] as i32) << 24)
  } else {
    0
  }
}

fn mat4_mul_vec4(matrix: &[[f32; 4]; 4], vector: &[f32; 4]) -> [f32; 4] {
  let mut result = [0.0; 4];

  for i in 0..4 {
    for (j, value) in vector.iter().enumerate() {
      result[i] += matrix[i][j] * value;
    }
  }

  result
}

pub fn parse_float(byte_array: &[u8; 4]) -> f32 {
  LittleEndian::read_f32(byte_array)
}

pub fn read(bytes: &mut Vec<u8>, len: i32) -> Vec<u8> {
  let mut result = vec![];

  for _ in 0..len {
    result.push(read_u8(bytes));
  }

  result
}

pub fn read_dict(bytes: &mut Vec<u8>, max_bytes: i32) -> HashMap<String, Vec<u8>> {
  let mut result = HashMap::new();
  let mut bytes_read = 0;

  if max_bytes == 0 {
    return result;
  }

  while let Some((key, value, key_value_length)) = read_dict_key_value(bytes) {
    result.insert(key, value);
    bytes_read += key_value_length;

    if bytes_read >= max_bytes {
      break;
    }
  }

  result
}

pub fn read_dict_key_value(mut bytes: &mut Vec<u8>) -> Option<(String, Vec<u8>, i32)> {
  let key_size = read_int(&mut bytes);
  if key_size > 0 {
    let key = read_str(bytes, key_size);
    let value_size = read_int(&mut bytes);

    let value = read(bytes, value_size);
    Some((key, value, key_size + value_size + 8))
  } else {
    None
  }
}

pub fn read_int(bytes: &mut Vec<u8>) -> i32 {
  let drain = bytes.drain(..4);
  let buffer = drain.as_slice();

  (buffer[0] as i32)
    + ((buffer[1] as i32) << 8)
    + ((buffer[2] as i32) << 16)
    + ((buffer[3] as i32) << 24)
}

pub fn read_str(bytes: &mut Vec<u8>, len: i32) -> String {
  let result = read(bytes, len);
  match str::from_utf8(&result) {
    Ok(string) => string.to_string(),
    _ => "".to_string(),
  }
}

pub fn read_u8(bytes: &mut Vec<u8>) -> u8 {
  let drain = bytes.drain(..1);
  let buffer = drain.as_slice();

  buffer[0]
}

fn vec3_dot(lhs: &[f32; 4], rhs: &[f32; 4]) -> f32 {
  lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
}

fn vec3_norm(vector: &[f32; 4]) -> f32 {
  let dot = vec3_dot(vector, vector);
  dot.sqrt()
}
