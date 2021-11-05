-- Your SQL goes here
CREATE TABLE ingredients (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	amount SMALLINT NOT NULL,
	category VARCHAR NOT NULL,
	price FLOAT NOT NULL
)
