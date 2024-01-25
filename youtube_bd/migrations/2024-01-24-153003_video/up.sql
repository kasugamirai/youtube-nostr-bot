CREATE TABLE videos (
    id SERIAL PRIMARY KEY,
    author VARCHAR NOT NULL,
    channel VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    link VARCHAR NOT NULL,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    userid INTEGER REFERENCES youtube_users(id)  NOT NULL
)