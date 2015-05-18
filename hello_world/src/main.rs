use std::cmp::Ordering;
// alternative to import enum variants
// use std::cmp::Ordering::{self, Equal, Less, Greater};

fn main() {
    print_number(5);
    print_sum(5, 6);
    println!("adding one to five: {}", add_one(5));
    println!("foo1: {}\nfoo2: {}", foo(4), foo(5));

    hello("Phil");

    let t: (i32, &str) = (1, "hello");
    let (x, y) = next_two(5);
    println!("x, y = {}, {}", x, y);

    let origin = Point { x: 0, y: 0 }; // origin: Point

    println!("The origin is at ({}, {})", origin.x, origin.y);

    let length = Inches(10);
    let Inches(integer_length) = length;
    println!("length is {} inches", integer_length);

    let ten  = Character::Digit(10);
    let four = Character::Digit(4);

    let ordering = cmp(x, y);
    if ordering == Ordering::Less {
        println!("less");
    } else if ordering == Ordering::Greater {
        println!("greater!");
    } else {
        println!("equal");
    }

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("something else"),
    }

    match cmp(x, y) {
       Ordering::Less => println!("less") ,
       Ordering::Greater => println!("greater"),
       Ordering::Equal => println!("equal"),
    }

    destructing();

    let mut s = 5; // mut x: u32
    let mut done = false; // mute done: bool

    for num in 1..10 {
        println!("{}", num);
    }

    while !done {
        s += s - 3;
        println!("{}", s);
        if s % 5 == 0 { done = true; }
    }

    s = 5;

    loop {
        s += s - 3;
        println!("{}", x);
        if s % 5 == 0 { break; }
    }
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn foo(x: i32) -> i32 {
    if x < 5 {
        x
    } else {
        x + 1
    }
}

fn diverges() -> ! {
    panic!("This function never returns!");
}

/// `hello` is a function that prints a greeting that is personalized
/// based on the name given.
///
/// # Arguments
///
/// * `name` - The name of the person you'd like to greet.
///
/// # Example
///
/// ```rust
/// let name = "Steve";
/// hello(name); // prints "Hello, Steve!"
/// ```
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn next_two(x: i32) -> (i32, i32) { (x + 1, x + 2) }

struct Point {
    x: i32,
    y: i32,
}

// tuple struct
struct Color(i32, i32, i32);

// tuple struct with one element called newtype
struct Inches(i32);

enum Character {
    Digit(i32),
    Other,
}

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

enum OptionalInt {
    Value(i32),
    Missing,
}

fn destructing() {
    let x = OptionalInt::Value(5);
    let y = OptionalInt::Missing;

    match x {
        OptionalInt::Value(n) => println!("x is {}", n),
        OptionalInt::Missing => println!("x is missing"),
    }

    match y {
        OptionalInt::Value(n) => println!("y is {}", n),
        OptionalInt::Missing => println!("y is missing"),
    }
}
