CREATE TYPE ingredient_category AS ENUM ('burger', 'topping', 'bun', 'sauce', 'side_dish', 'drink');

CREATE TABLE ingredients (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	amount INT NOT NULL,
	category ingredient_category NOT NULL,
	price FLOAT(2) NOT NULL
)
