mod proto;
use proto::{ProtoReader, ProtoWriter};

#[cfg(debug_assertions)]
fn main() {
    let mut proto_writer = ProtoWriter::new();
    proto_writer.write_constant64(1, 4);
    proto_writer.write_constant32(2, 5);
    proto_writer.write(5, |proto| {
        proto.write_constant64(1, 4);
        proto.write_constant32(2, 5);
        proto.write_string(3, "Hello, world!");
        proto.write(5, |proto| {
            proto.write_constant64(1, 4);
            proto.write_constant32(2, 5);
            proto.write_string(3, "Hello, world!");
        });
    });
    proto_writer.write_string(3, "Hello, world!");

    let proto_buffer = proto_writer.close();
    println!("proto_buffer: {:?}", proto_buffer);

    let proto_reader: ProtoReader = ProtoReader::new(proto_buffer).expect("Error creating ProtoReader");
    proto_reader.dump();
}
