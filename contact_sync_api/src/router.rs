

// #[cfg_attr(test, macro_use)]
// extern crate rocket;

// #[cfg_attr(test, macro_use)]
// extern crate rocket_contrib;

// #[cfg_attr(test, macro_use)]
// extern crate serde_derive;
use std::io::{self, Read};
use rocket_contrib::json::{Json, JsonValue};
use contact_sync_service;
use contact_sync_service::models::{User, NewUser, LoginInfo};
use contact_sync_service::models::{Contact, NewContact};
use contact_sync_service::models::{PhoneNumber, NewPhoneNumber};
use contact_sync_service::models::{Email, NewEmail};
use rocket::Outcome;
use rocket::data::Transform;
use rocket::Data;
use rocket::Request;
use rocket::response::status;
use rocket::response::content;
use serde::de::IntoDeserializer; 

#[post("/", data = "<new_user>")]
pub fn create_new_user(new_user: Json<NewUser>) -> status::Accepted<String> {
    
    let conn = contact_sync_service::establish_connection();
    
    let insert = NewUser { ..new_user.into_inner() };

    let return_user = contact_sync_service::create_user(
        &conn,
        insert.first_name, 
        insert.last_name,
        insert.user_name,
        insert.password
        );

    status::Accepted(Some(format!("user: '{}'", return_user.id)))
}

#[post("/login", data = "<login_user>")]
pub fn login(login_info: Json<LoginInfo>) -> status::Accepted<String> {
    let conn = contact_sync_service::establish_connection();

    let login = LoginInfo { ..login_info.into_inner() };
    
    let user = contact_sync_service::match_user(conn, login_info);
    
}

#[post("/", data = "<new_contact>")]
pub fn create_new_contact(new_contact: Json<NewContact>) -> status::Accepted<String> {
    let conn = contact_sync_service::establish_connection();
    // let insert: NewContact = serde_json::from_str(new_contact).unwrap();
    let insert = NewContact { ..new_contact.into_inner() };

    let return_contact = contact_sync_service::create_contact(
        &conn,
        insert.first_name,
        insert.last_name,
        insert.owner_id
    );

    status::Accepted(Some(format!("contact: '{}'", return_contact.id)))
}