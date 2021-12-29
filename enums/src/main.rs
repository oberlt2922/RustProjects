//simple enum
// fn main() {
//     let home = IpAddr::v4(String::from("127.0.0.1"));
//     let loopback = IpAddr::v6(String::from("::1"));
// }

// enum IpAddr {
//     v4(String),
//     v6(String)
// }

//implemet method on enum
// fn main() {
//     let w = Message::Write(String::from("hello"));
//     w.call();
//     Message::Move{x: 32, y: 23}.call();
//     Message::ChangeColor(123, 234, 345).call();
//     Message::Quit.call();
// }

// #[derive(Debug)]
// enum Message {
//     Quit,
//     Move{x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }

// impl Message {
//     fn call(&self) {
//         println!("{:#?}", self);
//     }
// }

//built in option enum
// fn main(){
//     let some_number = Some(5);
//     let some_string = Some("a string");
//     let absent_number: Option<i32> = None;
// }

fn main(){
    
}