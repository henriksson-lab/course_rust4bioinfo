-- Schema for the day-7 strain database.
-- Run with:  sqlite3 strains.db < data/schema.sql

CREATE TABLE strain (
    strain_id        INTEGER PRIMARY KEY,
    strain_name      TEXT    NOT NULL,
    species          TEXT    NOT NULL,
    isolation_source TEXT,
    year_isolated    INTEGER
);

CREATE TABLE assay (
    assay_id       INTEGER PRIMARY KEY,
    strain_id      INTEGER NOT NULL REFERENCES strain(strain_id),
    medium         TEXT    NOT NULL,
    od600_24h      REAL,
    date_measured  TEXT
);
