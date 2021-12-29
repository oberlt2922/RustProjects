//use std::io;

fn main() {
    //mutable vars
    // let mut x = 5;
    // println!("The value of x is {}", x);
    // x = 6;
    // println!("The value of x is {}", x);
    
    //shadowing immutable vars
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is {}", x);
    // }
    // println!("The value of x in the outer scope is {}", x);

    //tuple
    // let tup: (i32, f64, bool) = (25, 2.3, true);
    // let (x, y, z) = tup;
    // println!("destructured: The value of x is {}, the value of y is {}, the value of z is {}", x, y, z);
    // println!("structured: The value of x is {}, the value of y is {}, the value of z is {}", tup.0, tup.1, tup.2);

    //array
    // let arr = [3; 5];//five element arr where every elem is 3
    // let arr = [1, 2, 3, 4, 5];
    // println!("the third element is {}", arr[2]);
    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("the third element is {}", arr[2]);

    //append strings
    // let mut s = String::from("hello");
    // s.push_str(", world");

    //get user input
    // let mut s = String::new();
    // io::stdin()
    //     .read_line(&mut s)
    //     .expect("Failed to get line.");
    // let guess: u32 = match s.trim().parse() { //parse input
    //     Ok(num) => num,
    //     Err(_) => //error handling
    // };

    //string move
    // let x = String::from("hello");
    // let y = x;//x is no longer valid

    //copy string
    // let x = String::from("hello");
    // let y = x.clone();//both x and y are valid

    //error produced fix by either passing by reference, passing x.clone() or returning a string from new function into x
    // let x = String::from("hello");
    // new_function(x);
    // println!("{} from main function", x);
}

// fn new_function(s: String){
//     println!("{} from new function", s);
// }
