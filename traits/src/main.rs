#![allow(unused)]

use std::ops::Add;

// generics: der Punkt kann jeden Typ annehmen (x und y haben den gleichen Typ)
// für jeden Typ erstellt der Compiler eine Variante von Point
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// speziell für einen Typ
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// implementiert für alle Typen die addiert werden können
// bzw. die den Trait `Add` implementieren
//
// Clone dafür dass wir die Werte der Punkte addieren können,
// ohne besitzt der Punkte nehmen zu müssen
impl<T: Add + Clone> Point<T> {
    fn add(&self, other: &Point<T>) -> Point<T::Output> {
        Point {
            x: self.x.clone() + other.x.clone(),
            y: self.y.clone() + other.y.clone()
        }
    }
}

// unser eigener Trait
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

struct Article {
    author: String,
    link: String,
    views: i32
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

struct Tweet {
    author: String,
    content: String,
    likes: i32,
    retweets: i32
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("Tweet from {} ({} likes): {}",
            self.author, self.likes, self.content
        )
    }
}

fn main() {
    println!("Hello, world!");
}


// file:///mnt/d/Documents/repositories/rust-referat/iterators/src/main.rs
