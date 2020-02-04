-- Your SQL goes here
CREATE TABLE nyt (
   id SERIAL PRIMARY KEY,
   title VARCHAR NOT NULL,
   author VARCHAR NOT NULL,
   yield VARCHAR NOT NULL,
   time REAL NOT NULL,
   description TEXT NOT NULL,
   featured TEXT NOT NULL,
   tags TEXT NOT NULL,
   ratings INT,
   rating REAL,
   ingredients TEXT NOT NULL,
   steps TEXT NOT NULL,
   comments TEXT NOT NULL
)
