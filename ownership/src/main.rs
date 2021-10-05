fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // value of s moves into the function...
                                    // ... and s is thus invalid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function, but
                                    // type i32 implements the copy trait ...
                                    // ... and x is thus still valid here

}   // Here, x goes out of scope first, then s. But because the value of s was
    // moved, nothing special happens.

fn takes_ownership(some_string: String) {  // some_string comes into scope
    println!("{}", some_string);
}   // Here, some_string goes out of scope and 'drop' is called.
    // The backing memory is freed.

fn makes_copy(some_integer: i32) {          // some_integer comes into scope
    println!("{}", some_integer);
}   // Here, some_integer goes out of scope and nothing special happens.
