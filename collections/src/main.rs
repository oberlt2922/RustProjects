fn main() {
    //Vectors
    //let v: Vec<i32> = Vec::new();//empty vector
    // let mut v = vec![1, 2, 3];//initial i32 values
    // v.push(4);//add values
    // v.push(5);
    // v.push(6);
    // let third: &i32 = &v[2];//gets reference to value and crashes if out of bounds
    // match v.get(2){//gets Option<&T> variant and does not crash if out of bounds
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element")
    // };
    // let v2 = vec![1, 2, 3, 4, 5];
    // for i in &mut v{//iterate mutable vector
    //     *i += 1;//* is dereference operator
    // }
    // for i in &v2{//iterate immutable vector
    //     println!{"{}", i};
    // }

    //storing enums in vectors
    // enum SpreadSheetCell{
    //     Int(i32),
    //     Float(f32),
    //     Text(String)
    // }
    // let _v = vec![
    //     SpreadSheetCell::Int(3),
    //     SpreadSheetCell::Float(3.2),
    //     SpreadSheetCell::Text(String::from("Hello"))
    // ];
    // for i in &_v{
    //     match i {
    //         Int => println!("The element is an int"),
    //         Float => println!("The element is a float"),
    //         Text => println!("The element is a string")
    //     };
    // }
}
