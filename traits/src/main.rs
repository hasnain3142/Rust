pub struct Tweet {
    pub username: String,
    pub content: String,
}
pub struct Newsarticle {
    pub name: String,
    pub content: String,
}
pub trait Summary {
    fn summarize(&self) -> String
    {format!("Read more")}
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} tweeted {}",self.username,self.content)
    } }
impl Summary for Newsarticle  {
    fn summarize(&self) -> String {
        format!("{} wrote {}",self.name,self.content)
    }
}
fn main() {
    let tweet1 = Tweet{
        username: String::from("Ali"),
        content: String::from("Hello,World"),
    };
    let article1 = Newsarticle {
        name: String::from("John"),
        content: String::from("Hi World"),
    };
    println!("{}",tweet1.summarize());
    println!("{}",article1.summarize());

}
