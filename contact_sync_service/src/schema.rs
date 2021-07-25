table! {
    contacts (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        owner_id -> Int4,
    }
}

table! {
    emails (id) {
        id -> Int4,
        email_address -> Varchar,
        contact_id -> Int4,
    }
}

table! {
    phone_numbers (id) {
        id -> Int4,
        phone_number -> Varchar,
        number_type -> Varchar,
        contact_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        user_name -> Varchar,
        password -> Varchar,
    }
}

joinable!(contacts -> users (owner_id));
joinable!(emails -> contacts (contact_id));
joinable!(phone_numbers -> contacts (contact_id));

allow_tables_to_appear_in_same_query!(
    contacts,
    emails,
    phone_numbers,
    users,
);
