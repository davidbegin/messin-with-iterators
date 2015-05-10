#[allow(unused_variables)]

extern crate type_printer;

fn main() {
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

fn add_dem_pairs_up(vec_of_arrays: Vec<[i32; 2]>) -> Vec<i32> {
    let mut result = vec![];

    for arr in vec_of_arrays.iter() {
        result.push(arr[0]*arr[1]);
    }

    result
}
