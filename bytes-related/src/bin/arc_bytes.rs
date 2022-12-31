use std::sync::Arc;
use std::thread;
use bytes::Bytes;
/// 对于Arc<Bytes>的clone操作，不会clone内部数据，只是clone数据的指针
#[tokio::main]
async fn main() {
    let a = Bytes::from(b"abcdefgh".to_vec());
    let addr = a.as_ptr() as usize;

    // test the Bytes::clone is Sync by putting it in an Arc
    let a1 = Arc::new(a);
    let a2 = a1.clone();

    let t1 = thread::spawn(move || {
        let b: Bytes = (*a1).clone();
        assert_eq!(b.as_ptr() as usize, addr);
    });

    let t2 = thread::spawn(move || {
        let b: Bytes = (*a2).clone();
        assert_eq!(b.as_ptr() as usize, addr);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}