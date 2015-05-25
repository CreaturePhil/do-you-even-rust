pub fn if_else() {
    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            
            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, reduce by two");

            // This expression must turen `i32` as well.
            n / 2
        };

    println!("{} -> {}", n, big_n);
}

pub fn loops() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
       count += 1;

       if count == 3 {
           println!("three");

           // Skip the rest of this iteration
           continue;
       }

       println!("{}", count);

       if count == 5 {
           println!("OK, that's enough");

           // Exit this loop
           break;
       }
    }
}

pub fn nested() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break on the inner loop
            // break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

pub fn whiles() {
   // A counter variable
   let mut n = 1;

   // Loop while `n` is less than 101
   while n < 101 {
       if n % 15 == 0 {
           println!("fizzbuzz");
       } else if n % 3 == 0 {
           println!("fizz");
       } else if n % 5 == 0 {
           println!("buzz");
       } else {
           println!("{}", n);
       }

       // Increment counter
       n += 1;
   }
}

pub fn fors() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

// A function `age` which returns a `u32`.
fn age() -> u32 {
    15
}

pub fn matches() {
    let number = 13;

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13...19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    // Destructuring tuples
    let pair = (0, -2);

    println!("Tell me about {:?}", pair);
    // Match can be used to destructure a tuple
    match pair {
        // Destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        _      => println!("It doesn't matter what they are"),
    }

    println!("Tell me type of person you are");

    match age() {
        0             => println!("I'm not born yet I guess"),
        // Could `match` 1 ... 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 .. 12. Now the age can be reported.
        n @ 1  ... 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ... 19 => println!("I'm a teen of age {:?}", n),
        n             => println!("I'm an old person of age {:?}", n),
    }
}
