fn main() {
    _mixup_test();
    let wat = vec![10023, 31231];
    println!("{} is largest of {} and {}", largest(&wat), 31231, 10023)
}

struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V,W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

}

fn _mixup_test() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


// fn largest(lisT: &impl std::cmp::PartialOrd) -> T
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    /*
    In the body of largest we wanted to compare two values of type T using the greater than
    ( > ) operator. Because that operator is defined as a default method on the standard library
    trait std::cmp::PartialOrd , we need to specify PartialOrd in the trait bounds for T so
    the largest function can work on slices of any type that we can compare. We don’t need to
    bring PartialOrd into scope because it’s in the prelude. Change the signature of largest
    to look like this.

    The key line in this error is cannot move out of type [T], a non-copy slice . With our
    non-generic versions of the largest function, we were only trying to find the largest i32 or
    char . As discussed in the “Stack-Only Data: Copy” section in Chapter 4, types like i32 and
    char that have a known size can be stored on the stack, so they implement the Copy trait.
    But when we made the largest function generic, it became possible for the list
    parameter to have types in it that don’t implement the Copy trait. Consequently, we
    wouldn’t be able to move the value out of list[0] and into the largest variable, resulting
    in this error.
    */
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


/* Traits */
pub trait Summary {
    // Anything implementing this trait is forced to define this function
    // fn summarize(&self) -> String; // Would force users to implement this
    fn summarize(&self) -> String {
        // This becomes the default
        "Please get default stuff".to_string()
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
/*
Note that because we defined the Summary trait and the NewsArticle and Tweet types in
the same lib.rs in Listing 10-13, they’re all in the same scope. Let’s say this lib.rs is for a crate
we’ve called aggregator and someone else wants to use our crate’s functionality to
implement the Summary trait on a struct defined within their library’s scope. They would
need to bring the trait into their scope first. They would do so by specifying use
aggregator::Summary; , which then would enable them to implement Summary for their
type. The Summary trait would also need to be a public trait for another crate to implement
it, which it is because we put the pub keyword before trait in Listing 10-12.

One restriction to note with trait implementations is that we can implement a trait on a type
only if either the trait or the type is local to our crate. For example, we can implement
standard library traits like Display on a custom type like Tweet as part of our aggregator
crate functionality, because the type Tweet is local to our aggregator crate. We can also
implement Summary on Vec<T> in our aggregator crate, because the trait Summary is local
to our aggregator crate.

*/

fn _returns_summarizable() -> impl Summary {
    /*
    By using impl Summary for the return type, we specify that the returns_summarizable
    function returns some type that implements the Summary trait without naming the concrete
    type. In this case, returns_summarizable returns a Tweet , but the code calling this function
    doesn’t know that.

    However, you can only use impl Trait if you’re returning a single type. For example,
    code that returns either a NewsArticle or a Tweet with the return type specified as impl
    Summary wouldn’t work.
    */
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Of course, as you already know, people."),
        reply: false,
        retweet: false,
    }
}
