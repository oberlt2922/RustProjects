// use std::fmt::Display;

fn main() {
    let tweet = Tweet{
        username: String::from("test_username"),
        content: String::from("this is a tweet to test trait implementation"),
        reply: false,
        retweet: false
    };
    println!("{}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("The Sky Is Falling."),
        location: String::from("Chicago, IL"),
        author: String::from("Teddy Graham"),
        content: String::from("Traits are similar to interfaces")
    };
    notify(&article);
}

pub trait Summary {
    fn summarize(&self) -> String;
    //default implementation
    // fn summarize(&self) -> String {
    //     return String::from("(Read more...)");
    // }
}

//implement default behavior for trait
//impl Summary for NewsArticle {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

//a function that accepts any type that implements the Summary trait as an argument
//pub fn notify<T: Summary>(item: &T, item2: &T)//alternate signature syntax, both arguments must be same type
//pub fn notify(item: &(impl Summary + Display))//requires the objects to implement both Summary and Display
pub fn notify(item: &impl Summary) {
    println!("Breaking news!: {}", item.summarize());
}

//where clause
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// {      

// }

//returning types that implement traits
// fn returns_summarizable() -> impl Summary {
//     return Tweet {
//         username: String::from("Test_Username"),
//         content: String::from("Test Content"),
//         reply: false,
//         retweet: false
//     };
// }



// struct Pair<T> {
//     x: T,
//     y: T
// }

// impl<T> Pair<T> {
//     fn new (x: T, y: T) -> Self {
//         return Self{x, y};
//     }
// }

// impl <T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest number is x = {}", self.x);
//         }
//         else {
//             println!("The largest number is y = {}", self.y);
//         }
//     }
// }


//impl<T: Display> ToString for T//implements tostring for any trait that implements Display