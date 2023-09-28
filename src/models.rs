use crate::schema::accounts;

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a>{
    pub username:&'a str,
    pub email_address:&'a str,
    pub password:&'a str,
    pub in_use_by:&'a str
}
