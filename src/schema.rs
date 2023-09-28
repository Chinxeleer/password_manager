// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (account_id) {
        account_id -> Int4,
        #[max_length = 25]
        username -> Varchar,
        #[max_length = 50]
        email_address -> Varchar,
        #[max_length = 18]
        password -> Varchar,
        date_created -> Date,
        in_use_by -> Nullable<Array<Nullable<Text>>>,
    }
}
