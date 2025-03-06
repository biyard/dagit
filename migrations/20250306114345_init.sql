-- Add migration script here
-- Create the `agits` table
CREATE TABLE IF NOT EXISTS agits (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    title TEXT NOT NULL
);

-- Create the `collections` table
CREATE TABLE IF NOT EXISTS collections (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    agit_id BIGINT NOT NULL,
    FOREIGN KEY (agit_id) REFERENCES agits(id) ON DELETE CASCADE
);

-- Create the `artists` table
CREATE TABLE IF NOT EXISTS artists (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    title TEXT NOT NULL
);

-- Create the `artworks` table
CREATE TABLE IF NOT EXISTS artworks (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    agit_id BIGINT NOT NULL,
    FOREIGN KEY (agit_id) REFERENCES agits(id) ON DELETE CASCADE,
    collection_id BIGINT NOT NULL,
    FOREIGN KEY (collection_id) REFERENCES collections(id) ON DELETE CASCADE,
    artist_id BIGINT NOT NULL,
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE
);

-- Create the function to set `created_at` before insert
CREATE OR REPLACE FUNCTION set_created_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.created_at := NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create th
