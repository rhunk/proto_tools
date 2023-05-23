#![allow(dead_code)]

#[derive(Debug)]
pub struct BufferReader {
    buffer: Vec<u8>,
    pos: usize,
}

impl BufferReader {
    pub fn new(buffer: Vec<u8>) -> BufferReader {
        BufferReader {
            buffer,
            pos: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.buffer.len()
    }
    
    pub fn read_u8(&mut self) -> Option<u8> {
        if self.pos < self.buffer.len() {
            let byte = self.buffer[self.pos];
            self.pos += 1;
            Some(byte)
        } else {
            None
        }
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let low = self.read_u8()? as u16;
        let high = self.read_u8()? as u16;
        Some(low | (high << 8))
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        let low = self.read_u16()? as u32;
        let high = self.read_u16()? as u32;
        Some(low | (high << 16))
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        let low = self.read_u32()? as u64;
        let high = self.read_u32()? as u64;
        Some(low | (high << 32))
    }

    pub fn read_u128_le(&mut self) -> Option<u128> {
        let low = self.read_u64()? as u128;
        let high = self.read_u64()? as u128;
        Some(low | (high << 64))
    }

    pub fn read_var_u32(&mut self) -> Option<u32> {
        let mut result = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7f) as u32) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        Some(result)
    }

    pub fn read_var_u64(&mut self) -> Option<u64> {
        let mut result = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_u8()?;
            result |= ((byte & 0x7f) as u64) << shift;
            if byte & 0x80 == 0 {
                break;
            }
            shift += 7;
        }
        Some(result)
    }


    pub fn read_bytes(&mut self, length: usize) -> Option<Vec<u8>> {
        let mut bytes = Vec::new();
        for _ in 0..length {
            bytes.push(self.read_u8()?);
        }
        Some(bytes)
    }
}

pub struct BufferWriter {
    buffer: Vec<u8>,
}

impl BufferWriter {
    pub fn new() -> BufferWriter {
        BufferWriter {
            buffer: Vec::new(),
        }
    }

    pub fn get_buffer(&self) -> &Vec<u8> {
        &self.buffer
    }

    pub fn close(self) -> Vec<u8> {
        self.buffer
    }

    pub fn write_u8(&mut self, byte: u8) {
        self.buffer.push(byte);
    }

    pub fn write_u16(&mut self, value: u16) {
        self.write_u8((value & 0xff) as u8);
        self.write_u8((value >> 8) as u8);
    }

    pub fn write_u32(&mut self, value: u32) {
        self.write_u16((value & 0xffff) as u16);
        self.write_u16((value >> 16) as u16);
    }

    pub fn write_u64(&mut self, value: u64) {
        self.write_u32((value & 0xffffffff) as u32);
        self.write_u32((value >> 32) as u32);
    }

    pub fn write_u128(&mut self, value: u128) {
        self.write_u64((value & 0xffffffffffffffff) as u64);
        self.write_u64((value >> 64) as u64);
    }

    pub fn write_var(&mut self, mut value: u64) {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.write_u8(byte);
            if value == 0 {
                break;
            }
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u8(*byte);
        }
    }
}
