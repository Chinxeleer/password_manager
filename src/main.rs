#[macro_use]
extern crate diesel;

mod cli;
mod structs;
use clap::Parser;
use structs::*;
mod db;
mod models;
mod schema;

fn main() {
    let cli = Cli::parse();

    println!();
    println!("{:?}", cli);
}
