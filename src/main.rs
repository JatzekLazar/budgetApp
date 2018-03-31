extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
mod utils;
mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

fn main() {
    print!("{}", 256/100);
    println!("Hello, world!!");
}
