CREATE TYPE table_status AS ENUM ('available', 'in_use');

CREATE TABLE tables (
	id SERIAL PRIMARY KEY,
	"number" INT UNIQUE NOT NULL,
	status table_status NOT NULL
)
