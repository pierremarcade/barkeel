CREATE TABLE article_metas (
    article_id INTEGER REFERENCES articles(id),
    key VARCHAR(255),
    value TEXT,
    PRIMARY KEY (article_id)
);