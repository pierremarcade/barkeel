CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    published_at TIMESTAMP NOT NULL,
    author_id INTEGER REFERENCES users(id),
    homepage BOOLEAN DEFAULT false NOT NULL
);