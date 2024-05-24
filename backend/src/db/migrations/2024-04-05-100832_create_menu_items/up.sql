CREATE TABLE menu_items (
    id SERIAL PRIMARY KEY,
    menu_id INTEGER REFERENCES menus(id),
    article_id INTEGER REFERENCES articles(id),
    label VARCHAR(255) NOT NULL,
    position INTEGER NOT NULL
);