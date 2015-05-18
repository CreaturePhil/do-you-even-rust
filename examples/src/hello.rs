#[derive(Debug)]
struct Structure(i32);

/// **markdown** comment to generate docs
pub fn world() {
    println!("Hello World!");

    // formatting
    println!("{} days", 14);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {predicate}",
            predicate="over the lazy dog",
            subject="the quick brown fox",
            verb="jumps");
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
            "Slater",
            "Christian",
            actor="actor's");

    println!("Now {:?} will print!", Structure(3));
}

// comment

/*
 * block comment
 */

