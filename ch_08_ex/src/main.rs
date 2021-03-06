// TODO:
// [x] Given a list of integers, use a vector and return the mean (the average value), median (when
// sorted, the value in the middle position), and mode (the value that occurs most often; a hash
// map will be helpful here) of the list.
//
// [] Using a hash map and vectors, create a text interface to allow a user to add employee names to
// a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
// let the user retrieve a list of all people in a department or all people in the company by
// department, sorted alphabetically.
//
//

use std::collections::HashMap;

// Exercise 1 functions:
fn find_mean(int_list: &Vec<i32>) -> f64 {
    let int_list_sum: i32 = int_list.iter().sum();
    int_list_sum as f64 / int_list.len() as f64
}

fn find_median(int_list: &Vec<i32>) -> f64 {
    let mut list_clone = int_list.clone();
    list_clone.sort();
    let int_list_len = list_clone.len();

    if int_list_len % 2 == 0 {
        (list_clone[int_list_len / 2] + list_clone[(int_list_len / 2) - 1]) as f64 / 2.0
    } else {
        list_clone[int_list_len / 2] as f64
    }
}

fn find_mode(int_list: &Vec<i32>) -> i32 {
    let mut mode_map = HashMap::new();

    for num in int_list {
        *mode_map.entry(num).or_insert(0) += 1;
    }

    *mode_map
        .into_iter()
        .max_by_key(|&(_, v)| v)
        .map(|(v, _)| v)
        .unwrap()
}

// [x] Convert strings to pig latin. The first consonant of each word is moved to the end of the word
// and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
// to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
//
// Exercise 2 functions:
fn pig_latinify(s: &String) -> String {
    let phrase = s.clone().to_lowercase();
    let mut pl_phrase: Vec<String> = Vec::new();
    let iter = phrase.split_whitespace();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for i in iter {
        let mut word = i.chars();
        let first_char = word.next().unwrap();
        let mut pl_word = String::new();
        if vowels.contains(&first_char) {
            pl_word.push_str(format!("{}-hay", i).as_str());
            pl_phrase.push(pl_word);
        } else {
            pl_word.push_str(format!("{}-{}ay", word.as_str(), first_char).as_str());
            pl_phrase.push(pl_word);
        }
    }
    pl_phrase.join(" ")
}

fn main() {
    let int_list = vec![1, 3, 5, 4, 7, 3, 6, 4, 21, 65, 3];
    let int_list_mean = find_mean(&int_list);
    let int_list_median = find_median(&int_list);
    let int_list_mode = find_mode(&int_list);

    println!(
        "mean: {}, median: {}, mode {}",
        int_list_mean, int_list_median, int_list_mode
    );

    let s = String::from("What is going on my peeps");
    let pl_s = pig_latinify(&s);

    println!("{}", pl_s);
}
