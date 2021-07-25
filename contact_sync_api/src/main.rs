#![feature(plugin)]
// #![plugin(rocket_codegen)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod router;
use rocket_contrib::json::{Json, JsonValue};
use contact_sync_service;
use contact_sync_service::models::{User, NewUser};
use contact_sync_service::models::{Contact, NewContact};
use contact_sync_service::models::{PhoneNumber, NewPhoneNumber};
use contact_sync_service::models::{Email, NewEmail};




fn main() {
    rocket::ignite()
        .mount("/new_user", routes![router::create_new_user])
        .launch();
}
