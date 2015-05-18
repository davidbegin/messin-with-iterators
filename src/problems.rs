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
    // iterate over a range and print each value
    for x in 0..10 {
        println!("{}", x);
    }

    // iterate over a range an increase each number
    let result: Vec<i32> = (0..10).map(|i| i + 10).collect();
    println!("{:?}", result);

    // iterate over a range and filter out the even numbers
    let result: Vec<i32> = (0..10).filter(|i| i % 2 == 0).collect();
    println!("{:?}", result);
}
