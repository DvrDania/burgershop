CREATE TYPE order_status AS ENUM ('not_paid', 'in_queue', 'processing', 'fulfilled');

CREATE TABLE orders (
	id SERIAL PRIMARY KEY,
	table_id INT NOT NULL,
	status order_status NOT NULL,
	total FLOAT(2) NOT NULL,
	FOREIGN KEY (table_id) REFERENCES tables(id)
)
