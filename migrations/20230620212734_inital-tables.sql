-- Add migration script here
CREATE TABLE IF NOT EXISTS rooms(
    id SERIAL PRIMARY KEY,
    room_name VARCHAR(50) UNIQUE NOT NULL,
    room_owner VARCHAR(150) NOT NULL,
    is_private BOOLEAN NOT NULL,
    password VARCHAR(150)
);

CREATE TABLE IF NOT EXISTS messages(
    id SERIAL PRIMARY KEY,
    time_created TIMESTAMP NOT NULL,
    room INTEGER NOT NULL,
    body VARCHAR(2000) NOT NULL,
    sent_by VARCHAR(150) NOT NULL,
    CONSTRAINT room
        FOREIGN KEY (room) REFERENCES rooms (id)
    -- CONSTRAINT sender
    --     FOREIGN KEY (sent_by) REFERENCES users(nick);
);
