 fn main() {
//     //create instance
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1
//     };
//     //change value
//     user1.email = String::from("anotheremail@example.com");
//     //create instance with function
//     let mut user2 = build_user(String::from("someone2@example.com"), String::from("someusername234"));
//     //update instance based off of other instance with one value that is different
//     //moves instance from user1 to user3 because string value from user1 was moved
//     //user1 would be available if both string values were new values instead of moved from user1
//     let mut user3 = User {
//         email: String::from("someone3@example.com"),
//         ..user2
//     };
//     //tuple struct
//     struct Color(i32, i32, i32);
//     struct Point(i32, i32, i32);
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0, 0);
//     //Unit-like struct without any fields
//     struct AlwaysEqual;
//     let subject = AlwaysEqual;
 }

// //function to create instance
// fn build_user(email: String, username: String) -> User {
//     return User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1
//     }
// }

// //define struct
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
