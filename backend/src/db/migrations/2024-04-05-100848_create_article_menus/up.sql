CREATE TABLE article_menus (
    article_id INTEGER REFERENCES articles(id),
    menu_id INTEGER REFERENCES menus(id),
    PRIMARY KEY (article_id, menu_id)
);
