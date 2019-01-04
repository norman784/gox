use std::{
    io::{
        Read,
        Result,
    }
};

#[derive(Debug)]
pub struct Memory {
    pub buffer: Vec<u8>,
}

impl Read for Memory {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len();

        if len == 0 {
            Ok(0)
        } else if len > self.buffer.len() {
            // TODO: is this the way that is supposed to read the data?
            let mut buffer = vec![0; len];

            for (i, v) in self.buffer.iter().enumerate() {
                buffer[i] = *v;
            }

            buf.copy_from_slice(&buffer[0..len]);
            Ok(len)
        } else {
            buf.copy_from_slice(&self.buffer[0..len]);
            Ok(len)
        }
    }
}