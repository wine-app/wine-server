CREATE TYPE fruit_condition AS ENUM ('tart', 'ripe', 'jammy', 'baked');
CREATE TYPE wine_color AS ENUM ('white', 'red');

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
  color wine_color NOT NULL,
  producer VARCHAR REFERENCES producers(name) ON DELETE CASCADE NOT NULL,
  vintage INTEGER NOT NULL CHECK (vintage >= 1900),
  region VARCHAR REFERENCES regions(name) ON DELETE CASCADE NOT NULL,
  country VARCHAR REFERENCES countries(name) ON DELETE CASCADE NOT NULL,
  sparkling BOOLEAN NOT NULL,
  alcohol FLOAT NOT NULL,
  UNIQUE(name, producer, vintage)
);

CREATE TABLE red_analyses (
  id SERIAL PRIMARY KEY,
  wine_id INTEGER REFERENCES wines(id) ON DELETE CASCADE NOT NULL,
  red_fruit INTEGER NOT NULL CHECK (
    red_fruit >=0
    AND red_fruit <= 2
  ),
  black_fruit INTEGER NOT NULL CHECK (
    black_fruit >=0
    AND black_fruit <= 2
  ),
  blue_fruit INTEGER NOT NULL CHECK (
    blue_fruit >=0
    AND blue_fruit <= 2
  ),
  floral INTEGER NOT NULL CHECK (
    floral >=0
    AND floral <= 2
  ),
  vegetal INTEGER NOT NULL CHECK (
    vegetal >=0
    AND vegetal <= 2
  ),
  dried_herbs INTEGER NOT NULL CHECK (
    dried_herbs >=0
    AND dried_herbs <= 2
  ),
  mint INTEGER NOT NULL CHECK (
    mint >=0
    AND mint <= 2
  ),
  peppercorn INTEGER NOT NULL CHECK (
    peppercorn >=0
    AND peppercorn <= 2
  ),
  mocha INTEGER NOT NULL CHECK (
    mocha >=0
    AND mocha <= 2
  ),
  animalic INTEGER NOT NULL CHECK (
    animalic >=0
    AND animalic <= 2
  ),
  balsamic INTEGER NOT NULL CHECK (
    balsamic >=0
    AND balsamic <= 2
  ),
  organic INTEGER NOT NULL CHECK (
    organic >=0
    AND organic <= 2
  ),
  inorganic INTEGER NOT NULL CHECK (
    inorganic >=0
    AND inorganic <= 2
  ),
  oak INTEGER NOT NULL CHECK (
    oak >=0
    AND oak <= 2
  ),
  tannin INTEGER NOT NULL CHECK (
    tannin >=0
    AND tannin <= 3
  ),
  acid INTEGER NOT NULL CHECK (
    acid >=0
    AND acid <= 3
  ),
  alcohol INTEGER NOT NULL CHECK (
    alcohol >=0
    AND alcohol <= 3
  ),
  fruit_condition fruit_condition NOT NULL
);

CREATE TABLE white_analyses (
  id SERIAL PRIMARY KEY,
  wine_id INTEGER REFERENCES wines(id) ON DELETE CASCADE NOT NULL,
  apple INTEGER NOT NULL CHECK (
    apple >=0
    AND apple <= 2
  ),
  citrus INTEGER NOT NULL CHECK (
    citrus >=0
    AND citrus <= 2
  ),
  stone_fruit INTEGER NOT NULL CHECK (
    stone_fruit >=0
    AND stone_fruit <= 2
  ),
  tropical INTEGER NOT NULL CHECK (
    tropical >=0
    AND tropical <= 2
  ),
  floral INTEGER NOT NULL CHECK (
    floral >=0
    AND floral <= 2
  ),
  herbal INTEGER NOT NULL CHECK (
    herbal >=0
    AND herbal <= 2
  ),
  vegetal INTEGER NOT NULL CHECK (
    vegetal >=0
    AND vegetal <= 2
  ),
  botrytis INTEGER NOT NULL CHECK (
    botrytis >=0
    AND botrytis <= 2
  ),
  nutty INTEGER NOT NULL CHECK (
    nutty >=0
    AND nutty <= 2
  ),
  lees INTEGER NOT NULL CHECK (
    lees >=0
    AND lees <= 2
  ),
  buttery INTEGER NOT NULL CHECK (
    buttery >=0
    AND buttery <= 2
  ),
  organic INTEGER NOT NULL CHECK (
    organic >=0
    AND organic <= 2
  ),
  inorganic INTEGER NOT NULL CHECK (
    inorganic >=0
    AND inorganic <= 2
  ),
  wood INTEGER NOT NULL CHECK (
    wood >=0
    AND wood <= 2
  ),
  phenolic INTEGER NOT NULL CHECK (
    phenolic >=0
    AND phenolic <= 2
  ),
  sweetness INTEGER NOT NULL CHECK (
    sweetness >=0
    AND sweetness <= 2
  ),
  acid INTEGER NOT NULL CHECK (
    acid >=0
    AND acid <= 3
  ),
  alcohol INTEGER NOT NULL CHECK (
    alcohol >=0
    AND alcohol <= 3
  ),
  fruit_condition fruit_condition NOT NULL
);

CREATE TABLE compositions (
  wine_id INTEGER NOT NULL REFERENCES wines(id) ON DELETE CASCADE,
  grape VARCHAR NOT NULL REFERENCES grapes(name) ON DELETE CASCADE,
  percent INTEGER NOT NULL CHECK (
    percent >= 0
    AND percent <= 100
  ),
  PRIMARY KEY (wine_id, grape)
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  facebook_user_id VARCHAR UNIQUE,
  google_username VARCHAR UNIQUE,
  CONSTRAINT must_have_associated_authentication_account CHECK (facebook_user_id IS NOT NULL OR google_username IS NOT NULL)
);

CREATE TABLE reviews (
  user_id INTEGER REFERENCES users(id) ON DELETE CASCADE NOT NULL,
  wine_id INTEGER REFERENCES wines(id) ON DELETE CASCADE NOT NULL,
  liked BOOLEAN NOT NULL,
  PRIMARY KEY (user_id, wine_id)
);