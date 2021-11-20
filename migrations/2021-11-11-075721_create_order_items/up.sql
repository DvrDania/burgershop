CREATE TABLE order_items (
	id SERIAL PRIMARY KEY,
	order_id INT NOT NULL,
	total FLOAT(2) NOT NULL,
	FOREIGN KEY (order_id) REFERENCES orders(id)
)
