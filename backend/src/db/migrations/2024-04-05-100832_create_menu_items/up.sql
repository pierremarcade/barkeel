CREATE TABLE menu_items (
    id SERIAL PRIMARY KEY,
    menu_id INTEGER REFERENCES menus(id),
    label VARCHAR(255) NOT NULL,
    link VARCHAR(255) NOT NULL,
    position INTEGER NOT NULL
);