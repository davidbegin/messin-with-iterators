pub fn iterator_classes() {
    println!("3 components of iterators:");
    println!("\t * iterators give you a sequence of values");
    println!("\t * iterator adapters operate on an iterator and returns a new iterator");
    println!("\t * consumers operator on iterators and return a final set of values");

    consumers();
}

fn consumers() {
    let collected_values = vec![1, 2, 3].iter().map(|&i| i + 1).collect::<Vec<_>>();

    // vec![1, 2, 3].iter().map(|&i| i + 1);
    // when we pass x to find, it is borrowed, so we can't do a comparison against it
    //
    // so we have to deference it
    vec![1, 2, 3].iter().map(|&i| i + 1).find(|x| *x > 2);

    // find returns an option
    let the_one_i_was_looking_for = vec![1, 2, 3].iter().map(|&i| i + 1).find(|i| *i > 2);
    option_taker(the_one_i_was_looking_for);
}

fn option_taker(found_number: Option<i32>) {
    println!("\nunwrapping options like a boss: {}", found_number.unwrap());
}
