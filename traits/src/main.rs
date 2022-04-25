// use aggregator::{Summary,Tweet};
// fn main(){
//     let tweet=Tweet{
//         username:String::from("fgd"),
//         content:String::from("dhhf"),
//         rewrite:true,
//         reply:false
//     };
//     println!("{}",tweet.summerize());
// }
// pub trait Summary{
//     fn summerize(&self)->String;
// }
// pub struct NewArticles{
//     pub location:String,
//     pub headline:String,
//     pub author:String,
//     pub content:String
// }
// impl Summary for NewArticles{
//     fn summerize(&self)->String{
//         format!("{} by {} ({})",self.headline,self.author,self.location)
//     }
// }
// pub struct Tweet{
//     pub username:String,
//     pub content:String,
//     pub rewrite: bool,
//     pub reply:bool
// }
// impl Summary for Tweet{
//     fn summerize(&self)->String{
//         format!("{} by {}",self.content,self.username)
//     }
// }

pub trait Summary {
    fn summarize(&self) -> String;
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
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// use aggregator::{Summary, Tweet};

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
}
