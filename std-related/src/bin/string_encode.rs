

fn main() {
    let a = "ab".to_string();
    println!("{} {:?}", a.as_bytes().len(), a.as_bytes());

    println!("{}", String::from_utf8_lossy(a.as_bytes()));
}