// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn expected_minutes_in_oven() -> i32 {
    let minutes_in_oven = 40;
    minutes_in_oven
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let left_minutes_in_the_oven: i32 = 40 - actual_minutes_in_oven;
    left_minutes_in_the_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    let minutes_by_layers: i32 = number_of_layers * 2;
    minutes_by_layers
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    let mut total_preparation_time: i32 = number_of_layers * 2;
    total_preparation_time += actual_minutes_in_oven;
    total_preparation_time
}   