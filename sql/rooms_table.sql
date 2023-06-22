CREATE TABLE IF NOT EXISTS rooms(
    id SERIAL PRIMARY KEY,
    room_name VARCHAR(50) UNIQUE NOT NULL,
    room_owner VARCHAR(150) NOT NULL,
    is_private BOOLEAN NOT NULL,
    password VARCHAR(150)
)