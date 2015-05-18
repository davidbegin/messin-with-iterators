#![allow(unused_variables, dead_code)]

extern crate type_printer;
mod problems;

fn main() {
    problems::title();
    problems::main();
    warmups();
}

fn add_dem_pairs_up(vec_of_arrays: Vec<[i32; 2]>) -> Vec<i32> {
    vec_of_arrays.iter().map(|&arr| arr[0] * arr[1]).collect::<Vec<i32>>()
}

fn warmups() {
    // this is not valid,
    // we don't know the type
    // let first_vec = vec![];

    let second_vec: Vec<i32> = vec![];
    let _: Vec<i8> = vec![];
    let _: Vec<&str> = vec![];
    let _: Vec<String> = vec![];
    let _: Vec<[i32; 2]> = vec![];
    let _: Vec<[&str; 2]> = vec![];

    let all_yeah: Vec<[i32; 2]> = vec![[2, 3], [4, 5]];
    assert_eq!(add_dem_pairs_up(all_yeah), vec![6, 20]);
}
