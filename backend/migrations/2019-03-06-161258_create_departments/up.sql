-- Your SQL goes here
CREATE TABLE departments (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  abbreviation VARCHAR(255) NOT NULL
);
CREATE TABLE user_departments (
  id SERIAL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  department_id BIGINT NOT NULL
);