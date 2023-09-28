use clap::Subcommand;

use crate::structs::*;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Access accounts
    Account(UserCommand),
}

#[derive(Subcommand, Debug)]
pub enum UserSubCommand {
    /// Create a new user account
    Create(Account),
    /// Delete an existing user account
    Delete(DeleteAccount),
    /// Update an existing user account
    Update(UpdateAccount),
    /// Add to the account's list of sites
    AddSites(Site),
    /// show all users/ accounts
    Show,
}
