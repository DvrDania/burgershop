CREATE TABLE order_items (
	id SERIAL PRIMARY KEY,
	ingredient_id INT NOT NULL,
	order_id INT NOT NULL,
	amount INT NOT NULL,
	total FLOAT(2) NOT NULL,
	FOREIGN KEY (ingredient_id) REFERENCES ingredients(id),
	FOREIGN KEY (order_id) REFERENCES orders(id)
)
