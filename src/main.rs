#![feature(core)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate type_printer;
mod print_stuff;

use std::collections::VecDeque;
use std::collections::HashMap;

enum UserType { Regular, Guest, Admin }

struct User {
    name: String,
    user_type: UserType,
    cool_points: i32,
}

fn main() {
    let users = [
        User {
            name: "Bill".to_string(),
            user_type: UserType::Regular,
            cool_points: 100
        },
        User {
            name: "Steve".to_string(),
            user_type: UserType::Guest,
            cool_points: 0
        },
        User {
            name: "George".to_string(),
            user_type: UserType::Admin,
            cool_points: 100000
        }
    ];

    let most_points = users.iter().max_by(|&user| user.cool_points);
    println!(
        "Most Points: {:?} : {:?}",
        most_points.unwrap().name,
        most_points.unwrap().cool_points
    );

    let least_points = users.iter().min_by(|&user| user.cool_points);
    println!(
        "Least Points: {:?} : {:?}",
        least_points.unwrap().name,
        least_points.unwrap().cool_points
    );

}
