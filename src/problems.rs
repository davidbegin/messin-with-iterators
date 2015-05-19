extern crate type_printer;
// I suck at iterating
// I don't know how to use them
// every time I use them in one example it is not enough to stick
//
// I need to build some muscle memory and a deeper understanding

pub fn title() {
    println!("\nTraining for iter Karate match\n");
}

pub fn main() {
    problem_1();
}

fn problem_1() {
    // range_problems();
    vec_problems();
}

fn range_problems() {
    println!("\n~~~ Range Problems ~~~ \n");

    // iterate over a range and print each value
    for x in 0..10 {
        println!("{}", x);
    }

    // iterate over a range and increase each value
    let result: Vec<i32> = (0..10).map(|i| i + 10).collect();
    println!("{:?}", result);

    // iterate over a range and filter out the even numbers
    let result: Vec<i32> = (0..10).filter(|i| i % 2 == 0).collect();
    println!("{:?}", result);
}

fn vec_problems() {
    println!("\n~~~ Vec Problems ~~~ \n");

    // iterate over a vec and print off each value
    let subject = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for x in subject.iter() {
        println!("{}", x)
    }

    // iterate over a vec an increate each value
    let increased_subject: Vec<i32> = subject.iter().map(|&i| i + 10).collect();
    println!("{:?}", increased_subject);

    // iterate over a range and filter out the odd numbers
    let filtered_subject = increased_subject.iter().filter(|&i| !(*i % 2 == 0));
    println!("filtered list: {:?}", filtered_subject.collect::<Vec<&i32>>());

    // perform the same operation with a one-liner
    let subject = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = subject.iter()
        .map(|&i| i + 10)
        .filter(|i| !(i % 2 == 0))
        .collect::<Vec<i32>>();

    println!("filtered list: {:?}", result);
}
















