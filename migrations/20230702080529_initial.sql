CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- User storage
CREATE TABLE user (
	id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
	username TEXT NOT NULL UNIQUE,
	email TEXT NOT NULL UNIQUE,
	password TEXT NOT NULL,
	is_admin BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE user_token (
	token CHAR(64) PRIMARY KEY UNIQUE,
	user_id UUID REFERENCES users(id) ON DELETE CASCADE,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	last_used TIMESTAMP NOT NULL DEFAULT NOW()
);
-- End user storage

-- Recipe storage
CREATE TABLE recipe (
	id UUID PRIMARY KEY UNIQUE DEFAULT uuid_generate_v4(),
	title TEXT NOT NULL,
	description TEXT NOT NULL,
	author UUID REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE recipe_ingredient (
	recipe_id UUID REFERENCES recipe(id) ON DELETE CASCADE,
	num INTEGER NOT NULL,
	quantity NUMERIC(8, 2),
	unit TEXT,
	name TEXT,
	PRIMARY KEY (recipe_id, num)
);

CREATE TABLE recipe_step (
	recipe_id UUID REFERENCES recipe(id) ON DELETE CASCADE,
	num INTEGER NOT NULL,
	description TEXT NOT NULL,
	PRIMARY KEY (recipe_id, num)
);
-- End recipe storage

