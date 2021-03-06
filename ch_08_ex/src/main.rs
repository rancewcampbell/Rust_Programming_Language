// TODO:
// [x] Given a list of integers, use a vector and return the mean (the average value), median (when
// sorted, the value in the middle position), and mode (the value that occurs most often; a hash
// map will be helpful here) of the list.
//
// [] Convert strings to pig latin. The first consonant of each word is moved to the end of the word
// and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
// to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
//
// [] Using a hash map and vectors, create a text interface to allow a user to add employee names to
// a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
// let the user retrieve a list of all people in a department or all people in the company by
// department, sorted alphabetically.
//
//

use std::collections::HashMap;

fn find_mean(int_list: &Vec<i32>) -> f64 {
    let int_list_sum: i32 = int_list.iter().sum();
    int_list_sum as f64 / int_list.len() as f64
}

fn find_median(int_list: &Vec<i32>) -> f64 {
    let mut list_clone = int_list.clone();
    list_clone.sort();
    let int_list_len = list_clone.len();

    if int_list_len % 2 == 0 {
        (int_list[int_list_len / 2] + int_list[(int_list_len / 2) - 1]) as f64 / 2.0
    } else {
        int_list[int_list_len / 2] as f64
    }
}

fn find_mode(int_list: &Vec<i32>) -> &i32 {
    let mut mode_map = HashMap::new();

    for num in int_list {
        *mode_map.entry(num).or_insert(0) += 1;
    }

    mode_map
        .into_iter()
        .max_by_key(|&(_, v)| v)
        .map(|(v, _)| v)
        .unwrap()
}

fn ex1() {
    let int_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1];

    let mean = find_mean(&int_list);
    let median = find_median(&int_list);
    let mode = find_mode(&int_list);

    println!("mean: {}, median: {}, mode {:?}", mean, median, *mode);
}

fn main() {
    ex1();
}
