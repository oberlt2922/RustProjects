//matches
fn main() {
    let p = Coin::Penny;
    let n = Coin::Nickel;
    let d = Coin::Dime;
    let q = Coin::Quarter;
    println!("{}", value_in_cents(p));
    println!("nickel: {}", value_in_cents(n));
    println!("dime: {}", value_in_cents(d));
    println!("quarter: {}", value_in_cents(q));
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8{
    return match coin{
        Coin::Penny => {
            print!("penny: ");
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    };
}

//patterns that bind to values
// fn main() {
//     let q = Coin::Quarter(UsState::Illinois);
//     println!("{}", value_in_cents(q));
// }

// #[derive(Debug)]
// enum UsState {
//     Illinois,
//     Indiana
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState)
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     return match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin:: Quarter(state) => {
//             println!("State quarter from {:?}", state);
//             return 25;
//         }
//     };
// }

//matching with Option<T>
// fn main() {
//     let five = Some(5);
//     let _six = plus_one(five);
//     let _none = plus_one(None);
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     return match x{
//         Some(i) => Some(i + 1),
//         None => None
//     };
// }

// catch-alls
// let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         other => move_player(other),//catch-all that uses value
//         _ => reroll(),//catch-all that does not use value
//         _ => ()//catch-all that does nothing
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn move_player(num_spaces: u8) {}

//if let is similar to match but without exhaustive checking
// let mut count = 0;
// if let Coin::Quarter(state) = coin {
//     println!("State quarter from {:?}!", state);
// } else {
//     count += 1;
// }