struct Point<T> {
    x: T,
    y: T,
}

trait HasArea {
    fn area(&self) -> f64;
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) {
    println!("This shape has an area of {}", shape.area());
}

fn main() {
    //enum Option<T> {
        //Some(T),
        //None,
    //}

    let _x: Option<i32> = Some(5);
    let _int_origin = Point { x: 0, y: 0 };
    let _float_origin = Point { x: 0.0, y: 0.0 };

    let c = Circle {
        x: 0.0f64,
        y: 0.0f64,
        radius: 1.0f64,
    };

    let s = Square {
        x: 0.0f64,
        y: 0.0f64,
        side: 1.0f64,
    };

    print_area(c);
    print_area(s);
}

// this functions is generic over one type, T
// x has the type T
fn takes_anything<T>(x: T) {
    // do something with x
}
