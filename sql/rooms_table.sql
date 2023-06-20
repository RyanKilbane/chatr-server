CREATE TABLE IF NOT EXISTS rooms(
    id SERIAL PRIMARY KEY,
    room_name VARCHAR(50) UNIQUE NOT NULL,
    room_owner VARCHAR(50) NOT NULL,
    is_private BOOLEAN NOT NULL,
    password VARCHAR(150),
    CONSTRAINT room_owner
        FOREIGN KEY (room_owner) REFERENCES users (nick)
)