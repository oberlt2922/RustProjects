//changes in cargo.toml

// use std::fs;
// use std::fs::File;
// use std::io::ErrorKind;
// use std::io::{self, Read};

fn main() {
    //unrecoverable errors with panic!
    // panic!("crash and burn");

    //recoverable errors with Result
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error)
    // };

    //matching on different errors
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e)
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error)
    //     }
    // };

    //matching on different errors using closures
    // let f = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
    
    //unwrap returns the value from the ok variant of the Result enum or panics if there is an error
    //let f = File::open("hello.txt").unwrap();

    //expect does the same as unwrap except you get to set the error message
    //let f = File::open("hello.txt").expect("Failed to open hello.txt");


}

//propagating errors
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e)
//     };

//     let mut s = String::new();

//     return match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e)
//     };
// }

//propagating errors with ?
//the ? operator can only be used inside a function that returns a Result<T, E>
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;//value in Ok will be returned to f or Err will be returned from fn
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;//value in Ok will be returned to f
//     return Ok(s);
// }

//chaining method calls with ?
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     return Ok(s);
// }

//shortest way to write this function
// fn read_username_from_file() -> Result<String, io::Error> {
//     return fs::read_to_string("hello.txt");
// }
