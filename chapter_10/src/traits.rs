use crate::trait_bound::*;
pub struct NewsArticle {
    pub author : String,
    pub headline : String,
    pub content : String
}

pub struct Tweet{
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool
}

pub trait Summary{ 
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        format!("Read More From..{}",self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        self.username.clone()
    }
}

impl NewsArticle{
    fn extra_functionaly(&self) -> String {
        "____extra____func____".to_string()
    }
}

pub fn traits_use(){
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("{}\n{}",article.summarize(),article.extra_functionaly());
    notify(&article);
}