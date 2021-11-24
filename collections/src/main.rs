fn main() {
    let mut s1 = String::from("Hello, ");
    let mut s2 = String::from("world!");
    let s3 = s1 + &s2; // noote s1 has been moved here
                       // and can no longer be used
}
