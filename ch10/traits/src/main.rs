use std::{fmt::Display, iter::Sum};

use traits::{SocialPost, NewsArticle, Summary};

pub fn notify<T: Summary>(item: &T){ // syntax sugar for fn notify(item: &impl Summary)
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary + Display>(item: &T){ // syntax sugar for fn notify(item: &impl Summary + Display)
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3<T>(item: &T)
where 
T: Summary + Display, // this will fit with more than one trait bound
// S: Clone + Display, // you can add more trait bounds with commas
{
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary { // will only work if you return a single type that implements the Summary trait
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// fn returns_summarizable_wont_work(switch: bool) -> impl Summary { this won't work because it returns two different types
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("The Pittsburgh Penguins once again are the best hockey team in the
//     world."),
//         }
//     } else {
//         SocialPost {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

fn displayable<T: Display>(t: T) -> impl Display { t } // only know that it returns some type that implements Display
// doesn't matter what specific type it is, so it can not used as pass a String and the returned obj to call push_str

fn main(){
    let post = SocialPost{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new post: {}", post.summarize());

    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the
    world."),
    };
    println!("New article available: {}", article.summarize());
    notify2(&article);

    let s = String::from("hello");
    let mut s2 = displayable(s);
    // s2.push_str(" world");
    println!("{s2}");
}