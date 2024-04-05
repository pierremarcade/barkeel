CREATE TABLE role_permissions (
    role_id INTEGER REFERENCES roles(id),
    permission_id INTEGER REFERENCES permissions(id),
    PRIMARY KEY (role_id, permission_id)
);

INSERT INTO role_permissions (role_id, permission_id) VALUES
(1, 1), 
(1, 2), 
(1, 3),
(1, 4),
(1, 5),
(1, 6),
(1, 7),
(1, 8); 

INSERT INTO role_permissions (role_id, permission_id) VALUES
(2, 1),
(2, 2);