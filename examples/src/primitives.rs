use std::mem;

pub fn types() {
    let logical: bool = true;
    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    let mut mutable = 12; // Mutable `i32`

    println!("Types");
}

pub fn literals() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32- 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

// Tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

pub fn tuples() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64);

    // Values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32,), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one elment tuples, the comma is require to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));
}

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

pub fn arrays() {
   // Fixed-size array (type signature is superfluous) 
   let xs: [i32; 5] = [1, 2, 3, 4, 5];

   // All elements can be initialized to the same value
   let ys: [i32; 500] = [0; 500];

   println!("first element of the array: {}", xs[0]);
   println!("second element of the array: {}", xs[1]);

   println!("array size: {}", xs.len());

   // Arrays are stack allocated
   println!("array occupies {} byes", mem::size_of_val(&xs));

   // Arrays can be automatically borrowed as slices
   println!("borrow the whole array as a slice");
   analyze_slice(&xs);

   // Slices can point to a section of an array
   println!("borrow a section of the array as a slice");
   analyze_slice(&ys[1..4]);
}
