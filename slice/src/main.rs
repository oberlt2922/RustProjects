// iterate chars in string
// fn main() {
//     let s = String::from("Hello world!");
//     println!("result from function: {}", first_word(&s));
// }
// fn first_word(s: &str) -> &str {//putting &str makes this fn usable with &str and &String arguments
//     let bytes = s.as_bytes();//converts string to array of bytes
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     return &s[..];
// }

//string slice
// fn main(){
//     let s = String::from("hello world");
//     let hello = &s[..5];//from the first to the 4th char
//     let world = &s[6..];//from the 6th to the last char
//     let hello_world = &s[..]//slice of entire string
// }

//arr slice
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
