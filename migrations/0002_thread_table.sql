CREATE TABLE thread (
       id INT DEFAULT nextval('id') PRIMARY KEY,
       uid INT NOT NULL,
       subject VARCHAR NOT NULL,
       comment TEXT,
       board VARCHAR NOT NULL,
       ctime TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
       mtime TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
