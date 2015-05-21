#![allow(unused_variables, dead_code)]

extern crate type_printer;
mod problems;
mod deep_dive;
mod warmups;

fn main() {
    problems::title();
    // warmups::start();
    // problems::main();
    deep_dive::iterator_classes();
}
