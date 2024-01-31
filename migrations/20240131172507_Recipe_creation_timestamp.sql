ALTER TABLE recipes
ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW(),
ADD COLUMN edited_at TIMESTAMP NOT NULL DEFAULT NOW();

UPDATE recipes SET created_at = NOW(), edited_at = NOW();
