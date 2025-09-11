pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String {
        "someone did something".to_string()
    }

    // this whole thing works - commented due to complexities
    /*fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }*/
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

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/*
above is syntactic sugar for :
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
 */

/*
pub fn notify(item1: &impl Summary, item2: &impl Summary) {

Using impl Trait is appropriate if we want this function to allow item1 and item2 to have 
different types (as long as both types implement Summary). If we want to force both parameters t
o have the same type, however, we must use a trait bound, like this:

pub fn notify<T: Summary>(item1: &T, item2: &T) {

The generic type T specified as the type of the item1 and item2 parameters constrains the 
function such that the concrete type of the value passed as an argument for item1 and item2 
must be the same. */

/*
pub fn notify(item: &(impl Summary + Display)) {

The + syntax is also valid with trait bounds on generic types:

pub fn notify<T: Summary + Display>(item: &T) { */

/* 
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
we can use a where clause, like this:


fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}
*/

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}

// following code not allowed:
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        }
    }
} 
*/