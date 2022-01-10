fn main() {
    let result = largest(&vec![1, 5, 3, 2, 4]);
    println!("{:?}", result); 
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest{
            largest = item;
        }
    }
    return largest;
}


// fn main() {
//     let integer = Point{x: 2, y: 4};
//     let float = Point{x: 1.2, y: 4.3};
// }

//struct with generics
// struct Point<T> {
//     x: T,
//     y: T
// }

// fn main() {
//     let both_integer = Point{x: 1, y: 2};
//     let both_float = Point{x: 1.1, y: 2.2};
//     let different_types = Point{x: 1, y: 2.2};
// }

//struct with multiple generic data types
// struct Point<T, U> {
//     x: T,
//     y: U
// }

//generics in methods
// fn main() {
//     let p = Point{x: 5, y: 10};
//     println!("{}", p.x());
// }

// struct Point<T> {
//     x: T,
//     y: T
// }

// impl<T> Point<T> {//this method is available to all instances of point
//     fn x(&self) -> &T {
//         return &self.x;
//     }
// }

// impl Point<f32> {//this method is only available to instances of point where the type is f32
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

//implementing a method with two instances with different types
// fn main() {
//     let p1 = Point{x: 2, y: 4.3};
//     let p2 = Point{x: "Hello", y: 'c'};
//     let p3 = p1.mixup(p2);
//     println!("{:?}", p3);
// }

// #[derive(Debug)]
// struct Point<T, U>{
//     x: T,
//     y: U
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         return Point{
//             x: self.x,
//             y: other.y
//         }
//     }
// }