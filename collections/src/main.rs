fn main() {
    let mut v = Vec::new();
    // do stuff with v
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
} // <- v goes out of scope and is freed here
