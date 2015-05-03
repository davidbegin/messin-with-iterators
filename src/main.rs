#![allow(dead_code)]
#![allow(unused_variables)]

extern crate type_printer;

use std::collections::VecDeque;

fn main() {
    vec_duke_city();
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
