use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {}", third);

    match v4.get(2) {
        Some(third) => println!("The third elemnt is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v4 {
        println!("{}", i);
    }

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Creating a string
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    println!("{}", s);

    let mut s2 = String::from("foo");
    s2.push_str("bar");

    println!("{}", s2);

    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4);
    println!("s4 is {}", s4);

    let s5 = String::from("Hello, ");
    let s6 = String::from("world!");
    let s7 = s5 + &s6;

    // string concatenation with "+"
    let tic1 = String::from("tic");
    let tac1 = String::from("tac");
    let toe1 = String::from("toe");

    let s8 = tic1 + "-" + &tac1 + "-" + &toe1;

    println!("{}", s8);

    // string combining with "format!"
    let tic2 = String::from("tic");
    let tac2 = String::from("tac");
    let toe2 = String::from("toe");

    let s9 = format!("{}-{}-{}", tic2, tac2, toe2);

    println!("{}", s9);

    // basic hashmap implementation
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // collect method creates hashmap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    let text = "hello world wonderful world!";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
