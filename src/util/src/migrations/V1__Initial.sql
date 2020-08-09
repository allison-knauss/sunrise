CREATE TABLE Context (
  id SERIAL,
  name varchar(40) NOT NULL CHECK (name <> '')
);

CREATE TABLE Role (
  id SERIAL,
  name varchar(40) NOT NULL CHECK (name <> '')
);

CREATE TABLE UserAccount (
  id SERIAL,
  context_id integer,
  name varchar(40) NOT NULL CHECK (name <> ''),
  password varchar(40) NOT NULL CHECK (password <> '')
);
