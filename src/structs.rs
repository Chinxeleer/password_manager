use clap::{arg, command, Args, Parser};

use crate::cli::*;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Args, Debug)]
pub struct Account {
    #[arg(default_value_t=String::from("chinxeleer"),short,long)]
    /// username registered
    username: String,
    #[arg(short, long)]
    /// email address used in making the account
    email_address: String,
    #[arg(short, long)]
    /// the account's password
    password: String,
}

#[derive(Args, Debug)]
pub struct DeleteAccount {
    id: i32,
}

#[derive(Args, Debug)]
pub struct UpdateAccount {
    id: i32,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Args, Debug)]
pub struct Site {
    id: i32,
    site: Vec<String>,
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[command(subcommand)]
    command: UserSubCommand,
}
