#![allow(dead_code)]

use proto_tools::buffer_tools::{BufferReader, BufferWriter};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ProtoReader {
    values: BTreeMap<u32, Vec<(u8, Vec<u8>)>>,
}

impl ProtoReader {
    pub fn new(buffer: Vec<u8>) -> Option<ProtoReader> {
        let mut buffer_reader: BufferReader = BufferReader::new(buffer);
        let mut proto_reader = ProtoReader {
            values: BTreeMap::new(),
        };

        loop {
            if buffer_reader.is_empty() {
                break;
            }

            let tag = buffer_reader.read_var_u32()?;
            let id = tag >> 3;
            let wire_type: u32 = tag & 0x7;

            let constant: u64 = buffer_reader.read_var_u64()?;

            let value: Option<Vec<u8>> = match wire_type {
                0 => {
                    Some(constant.to_string().as_bytes().to_vec())
                }
                2 => {
                    Some(buffer_reader.read_bytes(constant as usize)?)
                },
                _ => None,
            };
            
            proto_reader.values.entry(id as u32).or_insert(Vec::new()).push((wire_type as u8, value?));
        }

        Some(proto_reader)
    }

    pub fn get_values(&self, id: u32) -> Option<&Vec<(u8, Vec<u8>)>> {
        self.values.get(&id)
    }

    pub fn get_single_value(&self, id: u32) -> Option<Vec<u8>> {
        let value = self.get_values(id);
        Some(value?.get(0)?.1.clone())
    }

    pub fn get_value_as_string(&self, id: u32) -> Option<String> {
        let value = self.get_single_value(id)?;
        Some(String::from_utf8(value).ok()?)
    }

    pub fn get_value_as_u32(&self, id: u32) -> Option<u32> {
        Some(self.get_value_as_string(id)?.parse::<u32>().ok()?)
    }

    pub fn get_value_as_u64(&self, id: u32) -> Option<u64> {
        Some(self.get_value_as_string(id)?.parse::<u64>().ok()?)
    }

    pub fn exists(&self, id: u32) -> bool {
        self.values.contains_key(&id)
    }

    pub fn read<T>(&self, id: u32, func: T) where T: Fn(ProtoReader) {
        let members = self.get_values(id);
        if members.is_none() {
            return;
        }

        for member in members.unwrap() {
            let reader = ProtoReader::new(member.1.clone());
            if reader.is_none() {
                continue;
            }
            func(reader.unwrap());
        }
    }

    #[cfg(debug_assertions)]
    pub fn dump(&self) {
        println!("ProtoReader dump:");
        for (id, values) in &self.values {
            println!("id: {}", id);
            for value in values {
                println!("    wire_type: {}", value.0);
                println!("    value: {:?}", value.1);
            }
        }
    }

}

pub struct ProtoWriter {
    writer: BufferWriter
}

impl ProtoWriter {
    pub fn new() -> ProtoWriter {
        ProtoWriter {
            writer: BufferWriter::new(),
        }
    }

    pub fn write_buffer(&mut self, index: u32, buffer: &Vec<u8>) {
        self.writer.write_var((index << 3 | 2) as u64);
        self.writer.write_var(buffer.len() as u64);
        self.writer.write_bytes(buffer);
    }

    pub fn write_constant64(&mut self, index: u32, constant: u64) {
        self.writer.write_var((index << 3) as u64);
        self.writer.write_var(constant);
    }

    pub fn write_constant32(&mut self, index: u32, constant: u32) {
        self.write_constant64(index, constant as u64)
    }

    pub fn write_string(&mut self, index: u32, string: &str) {
        self.write_buffer(index, &string.as_bytes().to_vec());
    }

    pub fn write<T>(&mut self, index: u32, func: T) where T: Fn(&mut ProtoWriter) {
        let mut writer = ProtoWriter::new();
        func(&mut writer);
        self.write_buffer(index, &writer.close());
    }

    pub fn close(self) -> Vec<u8> {
        self.writer.close()
    }
}