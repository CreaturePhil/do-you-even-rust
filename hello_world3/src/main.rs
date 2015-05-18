fn main() {
    println!("Hello, world!");

    //
    // Variables
    //

    let x = 5; // x: i32
    println!("x = {}", x);

    // y is a binding with the type i32 and the value five
    let y: i32 = 5;
    println!("y = {}", y);

    // mutable
    let mut z = 5;
    println!("z = {}", z);
    z = 10;
    println!("z = {}", z);

    //
    // Functions
    //

    print_number(5);

    print_sum(5, 6);

    println!("adding 1 to 5 is {}", add_one(5));

    // diverges();

    //
    // Primitive Types
    //

    // Booleans
    let t = true;
    let f: bool = false;
    println!("booleans are {} and {}", t, f);

    // Char
    let c = 'c';
    println!("a character is {}", c);

    // Arrays
    // Arrays have type [T; N].
    // The N is a compile-time constant, for the length of the array.
    let a = [1, 2, 3]; // a: [i32;, 3]
    let mut m = [1, 2, 3]; // m: [i32; 3]
    println!("a has {} elements", a.len());
    println!("m has {} elements", m.len());

    let b = [0; 20]; // a: [i32; 20]
    println!("b has {} elements", b.len());

    let names = ["Phil", "Stevo", "Mike"];
    println!("The second name is: {}", names[1]);

    // Slices
    let arr = [0, 1, 2, 3, 4];
    let middle = &arr[1..4]; // A slice of arr: just the elements 1, 2, and 3
    let complete = &arr[..]; // A slice containing all of the elements in arr

    // Tuple
    let t = (1, "hello");
    let u: (i32, &str) = (1, "hello");
    println!("t is ({}, {})", t.0, t.1);
    println!("u is ({}, {})", u.0, u.1);

    // destructuring let
    let (q, r, s) = (1, 2, 3);
    println!("q is {}", q);

    (0,); // single-element tuple
    (0); // zero in parentheses

    //
    // if
    //

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five!");
    }

    //
    // for Loops
    //

    println!("loop starts");
    for x in 0..10 {
        println!("{}", x); // x: i32
    }
    println!("loop ends");

    //
    // while loops
    //

    let mut done = false; // mut done: bool

    while !done {
        z += z - 3; 
        
        println!("{}", z);

        if x % 5 == 0 {
            done = true;
        }
    }

    //
    // Ownership
    //

    let k = Box::new(5);
    let l = add_five(k);

    println!("{}", l);

    let mut g = Box::new(5);

    add_ten(&mut g);

    println!("{}", g);

    let num = 5;

    match num {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("something else"),
    }
}

fn print_number(x: i32) {
    println!("x is {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is {}", x + y);
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}

// Diverging function
fn diverges() -> ! {
    panic!("This function never returns!");
}

fn add_five(mut num: Box<i32>) -> Box<i32> {
    *num += 5;

    num
}

// Borrowing
fn add_ten(num: &mut i32) {
    *num += 10;
}
