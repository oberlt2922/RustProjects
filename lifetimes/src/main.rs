//lifetimes
// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         return x;
//     }
//     else {
//         return y;
//     }
// }

//lifetimes with structs
fn main() {
    let novel = String::from("Call me Ishmael.  Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExerpt{part: first_sentence};
    println!("{}", i.part);
}

#[derive(Debug)]
struct ImportantExerpt<'a>{//an instance of this struct cannot outlive the reference that it is borrowing
    part: &'a str
}

//let s: &'static str = "I have static lifetime";//this variable lives until the end of the program

//using generics, traits, and lifetimes together
// use std::fmt::Display;

// fn longest_with_an_announcement<'a, T>(
//     x: &'a str,
//     y: &'a str,
//     ann: T,
// ) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
