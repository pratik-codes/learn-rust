// Traits
// A trait defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.
// Traits are similar to a feature often called interfaces in other languages, although with some differences.
// Traits are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

pub fn run() {
    println!("\n#################################");
    println!("Traits Module");
    println!("#################################\n");

    example();
    traist_bounds();
}

// EXAMPLE
// We want to make a media aggregator library crate named aggregator that can display summaries of data that might be stored in a NewsArticle or Tweet instance. To do this, we need a summary from each type, and we’ll request that summary by calling a summarize method on an instance. Listing 10-12 shows the definition of a public Summary trait that expresses this behavi

pub trait Summary {
    fn summarize(&self) -> String;

    // we can define default implementation for a trait
    fn summarize_default(&self) -> String {
        String::from("summary default")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implement summary for news article
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// we can implement summary for tweet as well but the implementation will be different.
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
    fn summarize_default(&self) -> String {
        format!("summary default for tweet")
    }
}

fn example() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // here we have custom implementation for tweet
    println!("1 new tweet: {}", tweet.summarize_default());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    // here we have default implementation for article as there were no custom implementation in  news article struct
    println!("New article available! {}", article.summarize_default());
}

fn traist_bounds() {
    // We can use trait bounds to specify that a generic type can be any type that has certain behavior.
    // Here we are using trait bounds to specify that the item must have a summarize method.
    // This is useful when we want to use a trait as a parameter to a function for a parameter type and this ensures that the type has the required behavior.
    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // or we can use the below syntax
    // pub fn notify<T: Summary>(item: &T) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // We can also specify more than one trait bound. Say we wanted notify to use display formatting as well as summarize on item: we specify in the notify definition that item must implement both Display and Summary. We can do so using the + syntax:
    // pub fn notify(item: &(impl Summary + Display)) {
    //     println!("Breaking news! {}", item.summarize());
    // }
    // or
    // pub fn notify<T: Summary + Display>(item: &T) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
}
