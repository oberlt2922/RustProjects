fn main() {
    let vowels: Vec<_> = "aeiou".chars().collect();
    let s = String::new();
    
    let first = s.chars().next();
    let mut translation: Option<String> = None;

    match first {
        Some(i) => {
            if vowels.contains(&i){
                translation = Some(format!("{}-hay", &s));
            }
            else {
                translation = Some(format!("{}-{}ay", &s[1..], &i));
            }
        },
        None => ()
    };

    match translation {
        Some(i) => {
            println!("{} becomes {}", s, i)
        },
        None => ()
    };
}
