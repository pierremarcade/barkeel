CREATE TABLE roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);
INSERT INTO roles (name) VALUES
('admin'),
('editor'),
('contributor'),
('subscriber');