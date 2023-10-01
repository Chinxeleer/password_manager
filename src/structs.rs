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
    pub username: String,
    #[arg(short, long)]
    /// email address used in making the account
    pub email_address: String,
    #[arg(short, long)]
    /// the account's password
    pub password: String,

    #[arg(short, long)]
    pub in_use_by: Vec<String>,
}

#[derive(Args, Debug)]
pub struct DeleteAccount {
    pub id: i32,
}

#[derive(Args, Debug)]
pub struct UpdateAccount {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Args, Debug)]
pub struct Site {
    pub id: i32,
    pub site: Vec<String>,
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[command(subcommand)]
    pub command: UserSubCommand,
}
