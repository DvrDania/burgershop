-- Your SQL goes here
CREATE TYPE ingredient_category AS ENUM ('burger', 'topping', 'bread', 'sauce', 'side_dish');

CREATE TABLE ingredients (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	amount INT NOT NULL,
	category ingredient_category NOT NULL,
	price FLOAT(2) NOT NULL
)
