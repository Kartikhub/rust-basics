pub trait Summary {
    // only declaring the method signature
    fn summarize(&self) -> String;

    // default implementation
    // fn default_summarize(&self) -> String {
    //     String::from("Read more....")
    // }

    // default implementation
    fn summarize_author(&self) -> String;

    fn default_summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
          format!("{}", self.author)
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as parameter
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Trait bound Syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}
// Multiple trait bounds
// pub fn notify(item: &(impl Summary + Display)) 
// pub fn notify<T: Summary + Display>(item: &T)

// Clearer trait bounds with where clause
// pub fn some_function<T, U>(t: &T, u: &U) -> i32 
// where 
//     T: Display + Clone,
//     U: Clone + Debug { 
//     }


fn main() {
    let tweet = Tweet {
        username: String::from("Crent"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.default_summarize());
    notify(&tweet);
    // println!("new tweet: {}", tweet.summarize())
}
