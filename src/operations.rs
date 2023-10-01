use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    db::establish_connection,
    models::{NewAccount, UserAccount},
    structs::{Account, DeleteAccount, Site, UpdateAccount, UserCommand},
};

pub fn handle_user_operations(account: UserCommand) {
    match account.command {
        crate::cli::UserSubCommand::Create(user) => create_user(user),
        crate::cli::UserSubCommand::Delete(user) => delete_account(user),
        crate::cli::UserSubCommand::Update(user) => update_account(user),
        crate::cli::UserSubCommand::AddSites(user) => add_site(user),
        crate::cli::UserSubCommand::Show => show_accounts(),
    }
}

fn create_user(user: Account) {
    println!("{:?}", user);
    use crate::schema::accounts;

    let mut connection = establish_connection();
    let new_user = NewAccount {
        username: &user.username,
        email_address: &user.email_address,
        password: &user.password,
        in_use_by: user.in_use_by.iter().map(|x| x as &str).collect(),
    };

    let value_returned = diesel::insert_into(accounts::table)
        .values(&new_user)
        .returning(UserAccount::as_returning())
        .get_result(&mut connection)
        .expect("Error saving new post");
    println!("{:?}", value_returned);
}

fn delete_account(user: DeleteAccount) {
    // if account exists delete it
    // if account doesn't exist panic!
    use crate::schema::accounts::dsl::*;

    let mut connection = establish_connection();
    let num_deleted = diesel::delete(accounts.filter(account_id.eq(user.id)))
        .execute(&mut connection)
        .expect("Error deleting post");

    print!("{:?}", num_deleted);
}

fn update_account(user: UpdateAccount) {
    // check if account to be updated exists
}

fn add_site(user: Site) {
    // check if account exists using the id
}

fn show_accounts() {
    use crate::schema::accounts::dsl::*;

    let mut connection = establish_connection();
    let accounts_returned = accounts
        .load::<UserAccount>(&mut connection)
        .expect("Error loading accounts");
    print!("\n");

    if accounts_returned.is_empty() {
        println!("No accounts created");
    } else {
        for entries in &accounts_returned {
            println!("{:?}", entries);
        }
    }
}
