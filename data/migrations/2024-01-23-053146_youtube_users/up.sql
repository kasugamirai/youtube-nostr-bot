CREATE TABLE youtube_users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    avatar VARCHAR  NULL,
    publickey VARCHAR NOT NULL,
    privatekey VARCHAR NOT NULL,
    channel VARCHAR NOT NULL,
    channel_id VARCHAR NOT NULL
)
