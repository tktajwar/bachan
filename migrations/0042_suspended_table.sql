CREATE TABLE suspended (
       id SERIAL PRIMARY KEY,
       uid INT NOT NULL,
       mod_id INT NOT NULL REFERENCES mod(id),
       thread_or_reply_id INT NOT NULL,
       until TIMESTAMP NOT NULL,
       reason VARCHAR(256) NOT NULL,
       ctime TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
