#![warn(clippy::pedantic)]

mod backend;
mod document;
mod ui;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
