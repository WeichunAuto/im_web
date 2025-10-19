-- Add migration script here
-- workspace for users
CREATE TABLE IF NOT EXISTS workspace (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL UNIQUE,
    owner_id BIGINT NOT NULL REFERENCES users(id),
    create_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE USERS ADD COLUMN ws_id BIGINT REFERENCES workspace(id);

-- initial values
INSERT INTO workspace (id, name, owner_id) VALUES (0, 'ws-super', 0);
UPDATE users SET ws_id = 0 where id = 0;

-- alter users table to make ws_id not null
ALTER TABLE users ALTER COLUMN ws_id SET NOT NULL;
