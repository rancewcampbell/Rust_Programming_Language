use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

// without ? operator
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");
//
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };
//
//     let mut s = String::new();
//
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
//

// with ? operator
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

// chaining after ? operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    // panic!("crash and burn!");
    //let v = vec![1, 2, 3];

    //v[99];
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

    // unwrap() will return the value inside Ok or call panic
    // let f = File::open("hello.txt").unwrap();
    //
    // .expect() is similar to .unwrap() but lets us choose the error message
    // let f = File::open("hello.txt").expect("Failed to open the file");
}
