#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };

    println!("rect1 is {:?}", rect1);
}
