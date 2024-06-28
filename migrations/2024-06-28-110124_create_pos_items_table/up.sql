-- Your SQL goes here
CREATE TABLE pos_items (
    id SERIAL PRIMARY KEY,
    code VARCHAR(255) UNIQUE NOT NULL,
    description VARCHAR(255),
    category VARCHAR(255),
    sub_category VARCHAR(255)
);
