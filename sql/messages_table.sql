CREATE TABLE IF NOT EXISTS messages(
    id SERIAL PRIMARY KEY,
    time_created TIMESTAMP NOT NULL,
    room INTEGER NOT NULL,
    body VARCHAR(200) NOT NULL,
    sent_by VARCHAR(150) NOT NULL,
    CONSTRAINT room
        FOREIGN KEY (room) REFERENCES rooms (id);
    CONSTRAINT sender
        FOREIGN KEY (sent_by) REFERENCES users(nick);
)