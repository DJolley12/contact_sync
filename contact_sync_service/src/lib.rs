#![feature(proc_macro_hygiene, decl_macro)]

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
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha3::Sha3;

use rocket_contrib::json::{Json, JsonValue};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use core::num;
use std::env;
use rocket_contrib::json;
use self::models::{User, NewUser, LoginInfo, AuthenticatedUser};
use self::models::{Contact, NewContact};
use self::models::{PhoneNumber, NewPhoneNumber};
use self::models::{Email, NewEmail};
use self::schema;

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
        password: &hash_password(password),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user.")
}

pub fn match_user<'a>(conn: &PgConnection, login_info: LoginInfo) -> User {
    use schema::users::dsl::*;


    let mut db_user_vec: Vec<User> = 
        users.filter(user_name.eq(login_info.username))
        .limit(1)
        .load::<User>(conn)
        .expect("Unable to login");
    let username = login_info.username.to_string();
    // match (login_info.username, login_info.password) {
    //     (Some(user_name), Some(password)) => {
    //         ...
    //     }
    // }
    
    let db_user = db_user_vec[0];

    return db_user;
}

pub fn login<'a>(conn: &PgConnection, login_attempt: LoginInfo, db_user: User) -> Option<AuthenticatedUser> {
    use schema::users::dsl::*;

    let login_attempt_pass = hash_password(login_attempt.password);
    let login_attempt_user = login_attempt.username.to_string();

    if login_attempt_pass == db_user.password && login_attempt_user == login_attempt_user {
        let authenticated_user = AuthenticatedUser {id: db_user.id};
        Some(authenticated_user)
    } else {
        return None;
    }
        
}

fn hash_password<'a>(password: &'a str) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

pub fn create_contact<'a>(conn: &PgConnection, first_name: &'a str, last_name: &'a str, owner_id: i32) -> Contact {
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