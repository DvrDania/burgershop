CREATE TYPE table_status AS ENUM ('available', 'in_use');

CREATE TABLE tables (
	"number" INT PRIMARY KEY NOT NULL,
	status table_status NOT NULL
)
