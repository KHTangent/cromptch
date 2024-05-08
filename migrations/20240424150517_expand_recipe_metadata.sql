ALTER TABLE recipes
ADD COLUMN time_estimate_active NUMERIC(8, 2),
ADD COLUMN time_estimate_total NUMERIC(8, 2),
ADD COLUMN source_url TEXT;

