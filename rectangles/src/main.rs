//without data structures
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("The area of the rectangle is {}", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32{
//     return width * height;
// }

//with tuples
// fn main() {
//     let rectangle = (30, 50);
//     println!("The area of the rectangle is {}", area(rectangle));
// }

// fn area(rectangle: (u32, u32)) -> u32 {
//     return rectangle.0 * rectangle.1;
// }

//with structs
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// fn main() {
//     let rectangle = Rectangle {
//         width: 30,
//         height: 50
//     };
//     println!("The area of the rectangle is {}", area(&rectangle));
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     return rectangle.width * rectangle.height;
// }

//debug derived trait
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }
// fn main() {
//     let rectangle = Rectangle {
//         width: 30,
//         height: 50
//     };
//     println!("rectangle is {:#?}", rectangle);
//     dbg!(&rectangle);
// }

//method
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle { //if method requiers instance, include &self as parameter and call instance.area()
//     fn area(&self) -> u32 {
//         return self.width * self.height;
//     }
// }

// fn main() {
//     let rectangle = Rectangle {
//         width: 30,
//         height: 50
//     };
//     println!("The area of the rectangle is {}", rectangle.area());
// }

//Constructor
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Rectangle{//if method does not require instnce, do not include self and call with Rectangle::method_name()
        return Rectangle {
            width: size,
            height: size
        };
    }
}

fn main() {
    let rectangle = Rectangle::square(3);
    println!("width: {} height: {}", rectangle.width, rectangle.height);
}
