CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- User storage
CREATE TABLE users (
	id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
	username TEXT NOT NULL UNIQUE,
	email TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL,
	is_admin BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE user_tokens (
	token CHAR(64) PRIMARY KEY UNIQUE,
	user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	last_used TIMESTAMP NOT NULL DEFAULT NOW()
);
-- End user storage

-- Recipe storage
CREATE TABLE recipes (
	id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
	title TEXT NOT NULL,
	description TEXT NOT NULL,
	author UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE recipe_ingredients (
	recipe_id UUID NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
	num INTEGER NOT NULL,
	quantity NUMERIC(8, 2) NOT NULL,
	unit TEXT NOT NULL,
	name TEXT NOT NULL,
	PRIMARY KEY (recipe_id, num)
);

CREATE TABLE recipe_steps (
	recipe_id UUID NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
	num INTEGER NOT NULL,
	description TEXT NOT NULL,
	PRIMARY KEY (recipe_id, num)
);
-- End recipe storage

