-- Add migration script here

-- create user table
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    fullname VARCHAR(64) NOT NULL,
    email VARCHAR(64) NOT NULL,
    password_hash VARCHAR(97) NOT NULL,
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- create chat type: single, private_channel, publish_channel
CREATE TYPE chat_type AS ENUM('single', 'private_channel', 'publish_channel');

-- create chats table
CREATE TABLE IF NOT EXISTS chats (
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL UNIQUE,
    type chat_type NOT NULL,

    -- user id list
    members BIGINT[] NOT NULL,
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- create messages table
CREATE TABLE IF NOT EXISTS messages (
    id BIGSERIAL PRIMARY KEY,
    chat_id BIGINT NOT NULL,
    sender_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    images TEXT[],
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (chat_id) REFERENCES chats(id),
    FOREIGN KEY (sender_id) REFERENCES users(id)
);

-- create index for the messages table: query messages by chat_id and create_at desc
CREATE INDEX IF NOT EXISTS chat_id_create_at_index ON messages(chat_id, create_at desc);

-- create index for the messages table: query messages by sender_id
CREATE INDEX IF NOT EXISTS sender_id_index ON messages(sender_id);