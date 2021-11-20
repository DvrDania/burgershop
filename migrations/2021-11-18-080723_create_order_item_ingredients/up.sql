-- Your SQL goes here
CREATE TABLE order_item_ingredients (
	id SERIAL PRIMARY KEY,
	order_item_id INT NOT NULL,
	ingredient_id INT NOT NULL,
	amount INT NOT NULL,
	total FLOAT(2) NOT NULL,
	FOREIGN KEY (ingredient_id) REFERENCES ingredients(id),
	FOREIGN KEY (order_item_id) REFERENCES order_items(id)
)
