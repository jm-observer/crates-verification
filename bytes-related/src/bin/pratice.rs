use bytes::{BufMut, BytesMut};

#[tokio::main]
async fn main() {
    let mut buf = BytesMut::with_capacity(10);
    buf.put_slice(&b"1234567890"[0..=9]);
    println!("len={} capacity={}", buf.len(), buf.capacity());

    let other = buf.split_off(3);
    println!("len={} capacity={} {:?}", buf.len(), buf.capacity(), buf);
    println!("len={} capacity={} {:?}", other.len(), other.capacity(), other);

}