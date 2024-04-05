CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT
);

INSERT INTO permissions (name, description) VALUES
('add_articles', 'Can publish articles'),
('edit_articles', 'Can edit articles'),
('delete_articles', 'Can delete articles'),
('manage_users', 'Can manage users'),
('get_users', 'Can get users'),
('edit_users', 'Can edit users'),
('delete_users', 'Can delete users'),
('add_users', 'Can add users');