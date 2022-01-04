fn main() {
    //creating new strings
    // let mut s = String::new();
    // let data = "initial contents";
    // let s = data.to_string();
    // let s = "initial contents".to_string();
    // let s = String::from("initial contents");

    //appending to strings with push_str
    // let mut s = String::from("foo");
    // s.push_str("bar");//takes string slice

    //concat string with +
    // let mut s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // s1 += &s2;
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //concat with format! macro
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = format!("{}-{}-{}", s1, s2, s3);//does not take ownership of strings

    //slicing strings
    // let hello = "hello";
    // let s = &hello[0..4];

    //iterate chars in string
    // for c in "hello".chars(){
    //     println!("{}", c);
    // }
}
