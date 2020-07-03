CREATE TABLE recipes (
   id SERIAL PRIMARY KEY,
   title TEXT NOT NULL,
   source INT NOT NULL,
   url TEXT NOT NULL,
   yields TEXT NOT NULL,
   time INT,
   description TEXT,
   steps TEXT[] NOT NULL,
   num_ratings INT NOT NULL,
   avg_rating REAL NOT NULL
);
CREATE TABLE tags (
   id SERIAL PRIMARY KEY,
   name TEXT NOT NULL
);
CREATE TABLE recipe_tag (
   id SERIAL PRIMARY KEY,
   recipes_id INT REFERENCES recipes(id),
   tags_id INT REFERENCES tags(id)
);
CREATE TABLE ingredients (
   id SERIAL PRIMARY KEY,
   name TEXT NOT NULL
);
CREATE TABLE sub_components (
   id SERIAL PRIMARY KEY,
   name TEXT NOT NULL
);
CREATE TABLE recipe_ingredient (
   id SERIAL PRIMARY KEY,
   recipes_id INT REFERENCES recipes(id),
   ingredients_id INT REFERENCES ingredients(id),
   sub_components_id INT REFERENCES sub_components(id),
   quantity REAL NOT NULL,
   quantity_note TEXT
);
CREATE TABLE comments (
   id SERIAL PRIMARY KEY,
   recipes_id INT NOT NULL REFERENCES recipes(id),
   body TEXT NOT NULL,
   votes INT NOT NULL
);
