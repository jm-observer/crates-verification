

fn main() {
    let mut a = u64::MAX;
    // panicked at 'attempt to add with overflow'
    // a += 1;

    // println!("{}", a);
    // 18446744073709551615 18446744073709551615
    println!("{} {}", a, a.saturating_add(1));
}
