CREATE TABLE youtube_users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    publickey VARCHAR NOT NULL,
    privatekey VARCHAR NOT NULL,
    channel VARCHAR NOT NULL
)
