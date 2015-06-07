fn main() {
    // for loop
    for x in 0..10 {
        print!("{} ", x);
    }

    println!("");

    // iterator is something we can called `.next()` method repeatedly
    // range is a iter. for example, 0..10
    let mut range = 0..10;

    loop {
        // next() returns an Option<i32>
        match range.next() {
            Some(x) => {
                print!("{} ", x);
            },
            None => { break }
        }
    }

    println!("");

    // Antipattern - Don't do this:
    //let nums = vec![1, 2, 3];
    //for i in 0..nums.len() {
        //print!("{} ", nums[i]);
    //}
    
    let nums = vec![1, 2, 3];

    for num in &nums {
        print!("{} ", num);
    }

    println!("");

    // another iter is `iter()`
    for num in nums.iter() {
        print!("{} ", num);
    }

    // Consumers operate on an iterator, producing some final set of values
    // most common is `collect()`
    // collect into Vec<i32>
    let _one_to_one_hundred = (1..101).collect::<Vec<i32>>();

    // another consumer is `find()`
    let greater_than_forty_two = (0..100)
                                .find(|x| *x > 42);

    match greater_than_forty_two {
        Some(_) => println!("\nWe got some numbers!"),
        None => println!("\nNo numbers found :("),
    }

    // fold consumer - fold(base, |accumulator, element| ...)
    let _sum = (1..4).fold(0, |sum, x| sum + x);

    // Iterator adapters take an iterator and modify it somehow, producing a new iterator
    // simplest one is `map`. for example, (1..100).map(|x| x + 1);
    
    for i in (1..100).filter(|&x| x % 2 == 0) {
        print!("{} ", i);
    }
}
