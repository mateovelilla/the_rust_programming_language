pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub trait Display {
    fn printing(&self) -> String{
        String::from("(Default print)")
    }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
impl Display for Tweet {}
pub fn notify<T, U>(t:&T, u:&U)
    where T: Summary + Display,
          U: Summary + Display
{
    println!("Breaking news! {}", t.summarize());
    println!("Breaking news! {}", u.printing());
}
// pub fn notify<T, U>(t:&T, u:&U) -> i32
//     where T: Summary + Display,
//           U: Summary + Display
// {
//     println!("Breaking news! {}", t.summarize());
//     println!("Breaking news! {}", u.printing());
//      6.0        
// }
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet, &tweet);
}
