CREATE TABLE videos (
                        id SERIAL PRIMARY KEY,
                        author VARCHAR NOT NULL,
                        title VARCHAR NOT NULL,
                        link VARCHAR NOT NULL,
                        published BOOLEAN NOT NULL DEFAULT FALSE
)
