
/*
    这个和c++中的接口定义一样,
    定义成pub的，别的mod也可以用了
    也可以定义实现
*/

pub trait Summery {
    // fn summerize(&self) ->String;
    fn summerize(&self) ->String {
        format!("this trait not implement...")
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

/*
impl Summery for NewsArticle {
    fn summerize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
*/
impl Summery for NewsArticle {

}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summery for Tweet {
    fn summerize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// 参数的意思是这个item是实现了Summery这个trait
pub fn notifyone(item: impl Summery) {
    println!("Breaking news! {}", item.summerize());
}

// 跟模板放在一起
pub fn notify<T: Summery>(item: T) {
    println!("Breaking News ! {}", item.summerize());
}

pub fn run() {
    let tweet = Tweet {
        username: String::from("oceanwavechina"),
        content: String::from("this is my first tweet"),
        reply: false,
        retweet: true,
    };

    let article = NewsArticle {
        headline: String::from("Big News !!"),
        location: String::from("Beijing ?"),
        author: String::from("liuyanan"),
        content: String::from("there will be no worry any more"),
    };

    println!("1 newsarticle: {}", article.summerize());
    println!("1 new tweet: {}", tweet.summerize());

    notify(tweet);

    notifyone(article);
}