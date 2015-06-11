fn main() {
    // &str called string slices - fixed size and immutable
    let _string = "Hello there."; // string: &'static str

    // String - head allocated string - growable and commonly created by
    // converting from a string slice using the `to_string` method
    let mut s = "Hello".to_string(); // mut s: String
    println!("{}", s);

    s.push_str(", world.");
    println!("{}", s);

    // Strings will coerce into &str with an &:
    takes_slice(&s);

    // there will be more bytes than chars
    let hachiko = "忠犬ハチ公";

    // print bytes
    for b in hachiko.as_bytes() {
        print!("{}, ", b);
    }

    println!("");

    // print chars
    for c in hachiko.chars() {
        print!("{}, ", c);
    }

    println!("");

    let _dog = hachiko.chars().nth(1); // kinda like hachiko[1]
}

// Viewing a String as a &str is cheap, but converting
// the &str to a String involves allocating memory.
fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}
