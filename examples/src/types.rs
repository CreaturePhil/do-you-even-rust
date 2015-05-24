// If you'd like to refer to the days without saying
// `Day::Monday`, then you use the following notation
// use Day::{
//     Monday,
//     Tuesday,
//     Wednesday,
//     Thursday,
//     Friday,
//     Saturday,
//     Sunday,
// }
//
// And you can refer to enum variants just like in C

// A unit struct
struct Nil;

// A tuple struct, which are, basically, named tuples
struct Pair(i32, f64);

// A struct with two field
struct Point {
    x: f64,
    y: f64,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

pub fn structs() {
    // Instantiate a `Point`
    let point: Point = Point { x: 0.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Destructure the point using `let` binding
    let Point { x: my_x, y: my_y } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        p1: Point { x: my_y, y: my_x },
        p2: point
    };

    // Instantiate a unit struct
    let _nil = Nil;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

mod my {
    // A public struct with public fields
    pub struct WhiteBox<T> {
        pub contents: T,
    }

    // A public struct with private fields
    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T
    }

    impl<T> BlackBox<T> {
        // A public constructor
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

pub fn visibility() {
   // Public structs with public fields can be constructed as usual
   let white_box = my::WhiteBox { contents: "public information" };

   // and their fields can be normally accessed
   println!("The white box contains: {}", white_box.contents);

   // structs with private fields can still be created using constructors
   let _black_box = my::BlackBox::new("classified information");
}

// enum with implicit discriminator (starts at 0)
#[allow(dead_code)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn mood(&self) {
        println!("{}", match *self {
            Day::Friday => "it's friday!",
            Day::Saturday | Day::Sunday => "weekend :-)",
            _ => "weekday...",
        })
    }
}

// enum with explicit discriminator
#[allow(dead_code)]
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn enums() {
    let today = Day::Monday;

    today.mood();

    // enums can be casted into integers
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

// Globals are declared outside all other scopes.
static LANGUAGE: &'static str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

pub fn constants() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
