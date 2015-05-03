#![allow(dead_code)]
#![allow(unused_variables)]

extern crate type_printer;

mod print_stuff;

use std::collections::VecDeque;
use std::collections::HashMap;

fn main() {
    let iterator = CountUp { current: 10 };
    let output = iterator.take(200).collect::<Vec<_>>();
    println!("{:?}", output);
}

struct CountUp {
    current: usize
}

impl Iterator for CountUp {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        Some(self.current)
    }
}


fn number_hash_mapper() {
    let mut numbers = HashMap::<i32, i32>::new();
    numbers.insert(1, 10);
    numbers.insert(2, 20);
    numbers.insert(3, 30);
    numbers.insert(5, 40);
    numbers.insert(5, 50);

    let iterator = numbers.iter();
    let mapped = iterator.map(|(&x, &y)| (x + 10, y + 100));
    let output = mapped.collect::<Vec<_>>();
    println!("{:?}", output);
}

fn hash_map_metropolis() {
    let mut cities = HashMap::<&str, &str>::new();
    cities.insert("Hyderabad", "India");
    cities.insert("Navacerrada", "Spain");
    cities.insert("Zabrze", "Poland");
    cities.insert("Ampfing", "Germany");

    print_stuff::seperator();

    for city in cities.iter() {
        println!("City Entry: {:?}", city);
    }

    print_stuff::seperator();

    println!("All Cities: {:?}", cities);
}

fn vec_duke_city() {
    let mut the_vec_duke_himself = VecDeque::new();
    the_vec_duke_himself.push_back(1);
    the_vec_duke_himself.push_back(7);
    the_vec_duke_himself.push_front(6);
    the_vec_duke_himself.push_front(4);
    let iterator = the_vec_duke_himself.iter();
    for i in iterator { println!("{:?}", i) }
}

fn clock() {
    let mut clock_cycles = 0;
    let mut clock_cycle = (0..60).cycle();

    loop {
        clock_cycles += 1;
        println!("NEXT: {:?}", clock_cycle.next().unwrap());
        if clock_cycles > 100 { println!("\n\nIma BREAK\n\n"); break }
    }
}
