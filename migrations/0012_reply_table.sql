CREATE TABLE reply (
       id INT DEFAULT nextval('id') PRIMARY KEY,
       uid INT NOT NULL,
       tid INT NOT NULL REFERENCES thread(id),
       comment TEXT NOT NULL,
       ctime TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
       redacted BOOLEAN DEFAULT FALSE NOT NULL
);
