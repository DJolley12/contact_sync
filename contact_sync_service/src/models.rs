use super::schema::contacts;
use super::schema::emails;
use super::schema::phone_numbers;
use super::schema::users;
use rocket_contrib::json;

#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub user_name: &'a str,
    pub password: &'a str,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct User {
        pub id: i32,
        pub first_name: String,
        pub last_name: String,
        pub user_name: String,
        pub password: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="contacts"]
pub struct NewContact<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub owner_id: &'a i32,
}

#[derive(Queryable, Deserialize)]
pub struct Contact {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub owner_id: i32,
}

#[derive(Insertable)]
#[table_name="phone_numbers"]
pub struct NewPhoneNumber<'a> {
    pub phone_number: &'a str,
    pub number_type: &'a str,
    pub contact_id: &'a i32,
}

#[derive(Queryable)]
pub struct PhoneNumber {
    pub id: i32,
    pub phone_number: String,
    pub number_type: String,
    pub contact_id: i32,
}


#[derive(Insertable)]
#[table_name="emails"]
pub struct NewEmail<'a> {
    pub email_address: &'a str,
    pub contact_id: &'a i32,
}

#[derive(Queryable)]
pub struct Email {
    pub id: i32, 
    pub email_address: String,
    pub contact_id: i32,
}