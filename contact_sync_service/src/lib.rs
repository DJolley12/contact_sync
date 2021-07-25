pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate serde_json;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use core::num;
use std::env;
use rocket_contrib::json;
use self::models::{User, NewUser};
use self::models::{Contact, NewContact};
use self::models::{PhoneNumber, NewPhoneNumber};
use self::models::{Email, NewEmail};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection:: establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, first_name: &'a str, last_name: &'a str, user_name: &'a str, password: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        first_name: first_name,
        last_name: last_name,
        user_name: user_name,
        password: password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user.")
} 

pub fn create_contact<'a>(conn: &PgConnection, first_name: &'a str, last_name: &'a str, owner_id: &'a i32) -> Contact {
    use schema::contacts;

    let new_contact = NewContact {
        first_name: first_name,
        last_name: last_name,
        owner_id: owner_id,
    };

    diesel::insert_into(contacts::table)
        .values(&new_contact)
        .get_result(conn)
        .expect("Error saving new contact.")
}

pub fn create_phone_number<'a>(conn: &PgConnection, phone_number: &'a str, number_type: &'a str, contact_id: &'a i32) -> PhoneNumber {
    use schema::phone_numbers;

    let new_phone_number = NewPhoneNumber {
        phone_number: phone_number,
        number_type: number_type, 
        contact_id: contact_id,
    };

    diesel::insert_into(phone_numbers::table)
        .values(new_phone_number)
        .get_result(conn)
        .expect("Error saving new phone number.")
}

pub fn create_email<'a>(conn: &PgConnection, email_address: &'a str, contact_id: &'a i32) -> Email {
    use schema::emails;

    let new_email = NewEmail {
        email_address: email_address, 
        contact_id: contact_id,
    };

    diesel::insert_into(emails::table)
        .values(new_email)
        .get_result(conn)
        .expect("Error saving new email.")
}