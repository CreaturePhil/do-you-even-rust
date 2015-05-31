pub fn vector() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Error! Immutable vectors can't grow
    //collected_iterator.push(0);

    // The `len` method yields the current size of the vector
    println!("Vector size: {}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Out of bounds indexing yields a panic
    //println!("Fourth element: {}", xs[3]);
}

pub fn strings() {
    // (all the type annotations are superfluous)
    // A reference to a string allocated in read only memory
//    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
//    println!("Pangram: {}", pangram);
//
//    // Iterate over words in reverse, no new string is allocated
//    for word in pangram.split_whitespace().rev() {
//        println!("> {}", word);
//    }

    // Copy chars into a vector, sort and remove duplicates
//    let mut chars: Vec<char> = pangram.chars().collect();
//    chars.sort();
//    chars.dedup();
//
//    // Create an empty and growable `String`
//    let mut string = String::new();
//    for c in chars {
//        // Insert a char at the end of string
//        string.push(c);
//        // Insert a string at the end of string
//        string.push_str(", ");
//    }
//
//    // The trimmed string is a slice to the original string, hence no new
//    // allocation is performed
//    let chars_to_trim: &[char] = &[' ', ','];
//    let trimmed_str: &str = string.trim_matches(chars_to_trim);
//    println!("Used characters: {}", trimmed_str);
//
//    // Heap allocate a string
//    let alice = String::from_str("I like dogs");
//    // Allocate new memory and store the modified string there
//    let bob: String = alice.replace("dog", "cat");
//
//    println!("Alice says: {}", alice);
//    println!("Bob says: {}", bob);
}
