use std::collections::HashMap;

fn main() {
    //constructing new hashmap
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    //constructing new hashmap using iterators
    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    //accessing values in hashmap
    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);//return Option<T>

    //iterating over hashmaps
    // for(key, value) in &scores {
    //     println!("{}: {}", key, value);
    // }

    //updating hashmaps
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);
    // println!("{:?}", scores);

    //only insert value if key has no value
    // scores.entry(String::from("Yellow")).or_insert(50);

    //count of each word in string
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
