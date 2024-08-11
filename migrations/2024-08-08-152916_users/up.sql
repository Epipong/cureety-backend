-- Your SQL goes here
CREATE TYPE roles
AS 
ENUM('admin', 'doctor', 'patient');

CREATE TABLE users (
  id          UUID NOT NULL PRIMARY KEY,
  username    VARCHAR(100) NOT NULL UNIQUE,
  email       VARCHAR(100) NOT NULL UNIQUE,
  hash        VARCHAR(122) NOT NULL, --argon hash
  role        ROLES NOT NULL DEFAULT 'patient',
  created_at  TIMESTAMP NOT NULL DEFAULT current_timestamp,
  updated_at  TIMESTAMP NOT NULL DEFAULT current_timestamp
);
