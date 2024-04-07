CREATE TABLE article_seos (
    article_id INTEGER REFERENCES articles(id),
    title_seo VARCHAR(255),
    description_seo TEXT,
    keywords_seo TEXT,
    PRIMARY KEY (article_id)
);