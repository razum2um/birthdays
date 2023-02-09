INSERT INTO $schema.birthdays(username, birthday) VALUES ($1, $2) ON CONFLICT (username) DO UPDATE SET birthday = EXCLUDED.birthday RETURNING $table_fields
