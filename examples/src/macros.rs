// This is the simplest macro, `say_hello` is the name of the macro
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument
    () => (
        // the macro will expand in the contents of this block
        println!("Hello!");
    )
}

pub fn hello() {
    // this call will expand into `println!("Hello");`
    say_hello!();
}

macro_rules! create_function {
    // this macro takes an argument of "type" `ident`
    // the `ident` designator is used for variable/function names
    ($func_name:ident) => (
        // this macro creates a function name `$func_name`
        fn $func_name() {
            // the stringify! macro converts an `ident` into a string
            println!("You called {:?}()", stringify!($func_name))
        }
    )
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // the `expr` designator is used for expressions
    ($expression:expr) => (
        // stringify! will conver the expression *as it is* into a string
        println!("{:?} = {:?}", stringify!($expression), $expression);
    )
}

pub fn funk() {
    foo();
    bar();

    print_result!(1u32 + 1);

    // remember that blocks are expressions
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

// macro_rules! is similar to a match block
macro_rules! test {
    // the arguments don't need to be separated by a comma
    // anny template can be used
    ($left:expr; and $right:expr) => (
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right);
    );
    // ^ each arm must be ended with a semicolon
    ($left:expr; or $right:expr) => (
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right); 
    );
}

pub fn overload() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false)
}
