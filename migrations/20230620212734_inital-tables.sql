-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
    nick VARCHAR(150) PRIMARY KEY,
    password VARCHAR(150) NOT NULL
);

CREATE TABLE IF NOT EXISTS rooms(
    id SERIAL PRIMARY KEY,
    room_name VARCHAR(50) UNIQUE NOT NULL,
    room_owner VARCHAR(150) NOT NULL,
    is_private BOOLEAN NOT NULL,
    password VARCHAR(150),
    CONSTRAINT owned_by
        FOREIGN KEY (room_owner) REFERENCES users(nick)
);

CREATE TABLE IF NOT EXISTS messages(
    id SERIAL PRIMARY KEY,
    time_created TIMESTAMP NOT NULL,
    room INTEGER NOT NULL,
    body VARCHAR(2000) NOT NULL,
    CONSTRAINT posted_in
        FOREIGN KEY (room) REFERENCES rooms(id)
);
