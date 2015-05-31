// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("destroying a box that contains {}", c);

    // `c` will be destroyed in this scope, and the memory will be freed
}

pub fn moves() {
    // Stack allocated integer
    let x = 5u32;

    // "Copy" `x` into `y`, there are no resources to move
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a heap allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // "Move" `a` into `b`
    // Here's what happens under the hood: the pointer `a` gets copied (*not*
    // the data on the heap, just its address) into `b`. Now both are pointers
    // to the *same* heap allocated data. But now, `b` *owns* the heap
    // allocated data; `b` is now in charge of freeing the memory in the heap.
    let b = a;

    // After the previous move, `a` can no longer be used
    // Error! `a` can no longer access the data, because its no longer owns the
    // heap memory
    // println!("a contains: {}", a);

    // "Move" `b` into the function; `b` gives up ownership of the heap data
    destroy_box(b);
}

pub fn mutability() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Hand over the box, changing the mutability
    let mut mutable_box = immutable_box;

    println!("mutable_box contained {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}

// This function takes ownership of the box
fn eat_box(boxed_int: Box<i32>) {
    println!("destroying box that contains {}", boxed_int);
}

// This function borrows an i32 instead
fn peep_inside_box(borrowed_int: &i32) {
    println!("This int is: {}", borrowed_int);
}

pub fn borrow() {
    // A boxed integer
    let boxed_int = Box::new(5);

    // Borrow the contents of the box, ownership is not taken
    peep_inside_box(&boxed_int);

    // The contents can be borrowed again
    peep_inside_box(&boxed_int);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_int: &i32 = &boxed_int;

        // Error! Can't destroy boxed_int, while the inner value has been
        // borrowed
        // eat_box(boxed_int);

        // `_ref_to_int` goes out of scope
    }

    // give up ownership of the box
    eat_box(boxed_int);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
   println!("I borrowed {} {} edition", book.title, book.year); 
}

// This function takes a reference to a mutable book
fn new_edition(book: &mut Book) {
    // the fields of the book can be modified
    book.year = 2014;
} 

pub fn borrow_mutability() {
    // An immutable Book
    let geb = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    // Immutably borrow `geb`
    borrow_book(&geb);

    // Error! Can't borrow an immutable object as mutable
    // new_edition(&mut geb);

    // `mutable_geb` is a mutable copy of `geb`
    let mut mutable_geb = geb;

    // Borrow a mutable object as mutable
    new_edition(&mut mutable_geb);

    // Mutable objects can be immutably borrowed
    borrow_book(&mutable_geb);
}

// When data is borrowed, it also freezes. Frozen data can't be modified via the
// original object, until all the references to it go out of scope.
fn freezing() {
    let mut _integer = 5i32;

    {
        // Borrow `integer`
        let _ref_to_integer = &_integer;

        // Error! `integer` is frozen in this scope.
        // _integer = 4;

        // `ref_to_integer` goes out of scope
    }

    // Ok! `integer` is not frozen in this scope
    _integer = 4;
}

struct Point { x: i32, y: i32, z:i32 }

pub fn alias() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let borrowed_point = &point;
        let another_borrow = &point;

        // Data can be accessed via the references and the original owner
        println!("Point has coordinates: ({}, {}, {})",
                borrowed_point.x, another_borrow.y, point.z);

        // Error! Can't borrow point as mutable because it's currenty
        // borrowed as immutable
        // let mutable_borrow = &mut point;

        // Immutable references go out of scope
    }

    {
        let mutable_borrow = &mut point;

        // Change data via mutable reference
        mutable_borrow.x = 5;

        // Error! Can't borrow `point` as immutable because it's currently
        // borrowed as mutable
        // let y = &point.y;

        // Error! Can't print, because println! takes an immutable reference
        // println!("Point Z coordinate is {}", point.z);

        // Mutable reference goes out of scope
    }

    // Immutable references to point are allowed again
    println!("Point now has coordinates: ({}, {}, {})",
            point.x, point.y, point.z);
}
