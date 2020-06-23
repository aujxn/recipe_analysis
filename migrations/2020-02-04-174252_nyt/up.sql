-- Your SQL goes here
CREATE TABLE nyt (
   id SERIAL PRIMARY KEY,
   title TEXT NOT NULL,
   author TEXT,
   yield TEXT NOT NULL,
   time TEXT,
   description TEXT,
   featured TEXT,
   tags TEXT[] NOT NULL,
   ratings INT NOT NULL DEFAULT 0,
   rating INT NOT NULL DEFAULT 0,
   sub_components TEXT[],
   indices INT[],
   ingredients TEXT[] NOT NULL,
   quantities TEXT[] NOT NULL,
   steps TEXT[] NOT NULL,
   comments TEXT[],
   comment_votes INT[],
   url_id TEXT NOT NULL
)
