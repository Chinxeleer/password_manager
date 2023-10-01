use crate::schema::accounts;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub email_address: &'a str,
    pub password: &'a str,
    pub in_use_by: Vec<&'a str>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name=crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserAccount {
    pub account_id: i32,
    pub username: String,
    pub email_address: String,
    pub password: String,
    pub date_created:NaiveDate,
    pub date_modified:Option<NaiveDateTime>,
    pub in_use_by: Option<Vec<Option<String>>>,
}

// correct the model so that it becomes a working model.
