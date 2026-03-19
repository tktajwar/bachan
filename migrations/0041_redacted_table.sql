CREATE TABLE redacted (
       id SERIAL PRIMARY KEY,
       thread_or_reply_id INT NOT NULL,
       mod_id INT NOT NULL REFERENCES mod(id),
       reason VARCHAR(256) NOT NULL,
       rtime TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
