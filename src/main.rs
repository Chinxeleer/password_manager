extern crate chrono;
extern crate diesel;

mod cli;
mod structs;
use clap::Parser;
use operations::*;
use structs::*;

mod db;
mod models;
mod operations;
mod schema;


fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Account(account) => handle_user_operations(account),
    }
}
