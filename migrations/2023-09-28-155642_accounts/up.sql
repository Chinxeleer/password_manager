
CREATE TABLE accounts(
    account_id serial PRIMARY KEY,
    username VARCHAR(25) NOT NULL,
    email_address VARCHAR(50) NOT NULL,
    password VARCHAR(18) NOT NULL,
    date_created DATE NOT NULL DEFAULT CURRENT_DATE,
    date_modified TIMESTAMP ,
    in_use_by TEXT[]
);

