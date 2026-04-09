CREATE TABLE PendingPost (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       is_thread BOOLEAN NOT NULL,
       uid INT NOT NULL,
       subject VARCHAR,
       comment TEXT NOT NULL,
       board VARCHAR REFERENCES board(url),
       tid INT REFERENCES thread(id),
       ctime TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
