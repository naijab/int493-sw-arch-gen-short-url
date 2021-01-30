CREATE DATABASE IF NOT EXISTS links_db;

CREATE TABLE IF NOT EXISTS links_db.short_urls(
    id BIGSERIAL PRIMARY KEY,
    short_url VARCHAR(10) NOT NULL,
    full_url TEXT NOT NULL,
    user_id integer NOT NULL DEFAULT '1'
);