extern crate reqwest;

use std::io::Read;

fn main() {
    println!("Hello World!");


    let mut resp = try!(reqwest::get("https://www.rust-lang.org")) {
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
        Ok(f) => f,
    };
    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content);
    // match test() {
    //     Ok(s) => println!("Error: {}", s.to_string()),
    //     Err(e) => println!("{}", e),
    // }
}

// fn test() {
//     let mut resp = try!(reqwest::get("https://www.rust-lang.org")?) {
//         Err(e) => {
//             println!("Error: {}", e);
//             return;
//         }
//         Ok(f) => f,
//     };
//     assert!(resp.status().is_success());
//
//     let mut content = String::new();
//     resp.read_to_string(&mut content);
// }
