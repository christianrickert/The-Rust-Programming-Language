fn main() {
    let (x, y, z) = (1, 2, 3);

    fn foo(x: i32) {
        // code goes here
    }

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);
}
