CREATE SCHEMA IF NOT EXISTS $schema;
CREATE TABLE IF NOT EXISTS $schema.birthdays (
		username   VARCHAR PRIMARY KEY NOT NULL,
		birthday   DATE NOT NULL
);
TRUNCATE TABLE $schema.birthdays;

