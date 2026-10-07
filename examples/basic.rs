use binary_codec::BinaryCodec;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, PartialEq)]
pub struct TestMessage {
    pub number: u16,
}

impl BinaryCodec for TestMessage {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u16(self.number);
    }

    fn decode(buf: &mut Bytes) -> Option<TestMessage> {
        let number = buf.get_u16();
        Some(Self { number })
    }
}

fn main() {
    let msg = TestMessage { number: 123 };

    let mut buf = BytesMut::new();
    msg.encode(&mut buf);

    let mut frozen = buf.freeze();
    let decoded = TestMessage::decode(&mut frozen).unwrap();

    println!("✅ Original: {:?}", msg);
    println!("✅ Decoded:  {:?}", decoded);
    assert_eq!(msg, decoded);
}
