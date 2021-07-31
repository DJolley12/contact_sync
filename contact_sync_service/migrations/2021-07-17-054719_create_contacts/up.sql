CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    user_name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    
);

CREATE TABLE contacts (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    owner_id INT NOT NULL,
    CONSTRAINT fk_contacts_owner
        FOREIGN KEY(owner_id) 
        REFERENCES users(id)
            ON DELETE CASCADE
);

CREATE TABLE phone_numbers (
  id SERIAL PRIMARY KEY,
  phone_number VARCHAR NOT NULL,
  number_type VARCHAR NOT NULL,
  contact_id INT NOT NULL,
  CONSTRAINT fk_phone_numbers_contact
        FOREIGN KEY(contact_id) 
        REFERENCES contacts(id)
            ON DELETE CASCADE
);

CREATE TABLE emails (
    id SERIAL PRIMARY KEY,
    email_address VARCHAR NOT NULL,
    contact_id INT NOT NULL,
    CONSTRAINT fk_emails_contact
        FOREIGN KEY(contact_id) 
        REFERENCES contacts(id)
            ON DELETE CASCADE
);