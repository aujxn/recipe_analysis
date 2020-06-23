-- Your SQL goes here
CREATE TABLE recipes_table (
   id SERIAL PRIMARY KEY,
   title VARCHAR NOT NULL,
   time REAL NOT NULL,
   yields INT NOT NULL,
   ingredients VARCHAR NOT NULL,
   instructions VARCHAR NOT NULL,
   rating REAL,
   reviews INT,
   url_id INT NOT NULL
)
