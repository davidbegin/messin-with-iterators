#![allow(dead_code)]
#![allow(unused_variables)]

extern crate type_printer;

fn main() {
    // clock();
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
