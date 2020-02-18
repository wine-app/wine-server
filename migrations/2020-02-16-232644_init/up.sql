CREATE TYPE wine_intensity AS ENUM ('delicate', 'moderate', 'powerful');
CREATE TYPE color AS ENUM ('white', 'red', 'rose');
CREATE TABLE grapes (
  name VARCHAR NOT NULL CHECK (name <> '') PRIMARY KEY
);
CREATE TABLE countries (
  name VARCHAR NOT NULL CHECK (name <> '') PRIMARY KEY
);
CREATE TABLE regions (
  name VARCHAR NOT NULL CHECK (name <> '') PRIMARY KEY
);
CREATE TABLE producers (
  name VARCHAR NOT NULL CHECK (name <> '') PRIMARY KEY
);
CREATE TABLE wines (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL CHECK (name <> ''),
  producer VARCHAR REFERENCES producers(name) NOT NULL ON DELETE CASCADE,
  vintage INTEGER NOT NULL CHECK (vintage >= 1900),
  region VARCHAR REFERENCES regions(name) NOT NULL ON DELETE CASCADE,
  country VARCHAR REFERENCES countries(name) NOT NULL ON DELETE CASCADE,
  sparkling BOOLEAN NOT NULL,
  sweetness INTEGER NOT NULL CHECK (
    sweetness >= 0
    AND sweetness <= 5
  ),
  tannin INTEGER NOT NULL CHECK (
    tannin >= 0
    AND tannin <= 5
  ),
  acid INTEGER NOT NULL CHECK (
    acid >= 0
    AND acid <= 5
  ),
  alcohol INTEGER NOT NULL CHECK (
    alcohol >= 0
    AND alcohol <= 5
  ),
  body INTEGER NOT NULL CHECK (
    body >= 0
    AND body <= 5
  ),
  intensity wine_intensity NOT NULL,
  UNIQUE(name, producer, vintage)
);
CREATE TABLE compositions (
  wine_id INTEGER NOT NULL REFERENCES wines(id) ON DELETE CASCADE,
  grape VARCHAR NOT NULL REFERENCES grapes(name) ON DELETE CASCADE,
  percent INTEGER NOT NULL CHECK (
    percent >= 0
    AND percent <= 100
  ),
  PRIMARY KEY(wine_id, grape)
);
CREATE TABLE tasting_notes (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL CHECK (name <> ''),
  UNIQUE(name)
);