#[derive(Debug)]
enum CadProvince {
    BC,
    AB,
    SK,
    MB,
    ON,
    QC,
    NB,
    NL,
    NT,
    NU,
    PE,
    YT,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(CadProvince),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Provincial quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let quarter = Coin::Quarter(CadProvince::AB);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    value_in_cents(quarter);

    //    let some_u8_value = 7u8;
    //    match some_u8_value {
    //        1 => println!("one"),
    //        3 => println!("three"),
    //        5 => println!("five"),
    //        7 => println!("seven"),
    //        _ => (),
    //    }
    let some_u8_value = Some(3u8);
    //    match some_u8_value {
    //        Some(3) => println!("three"),
    //        _ => (),
    //    }
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
