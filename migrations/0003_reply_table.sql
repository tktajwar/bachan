CREATE TABLE reply (
       id INT DEFAULT nextval('id') PRIMARY KEY,
       uid INT NOT NULL,
       tid INT NOT NULL,
       comment TEXT NOT NULL,
       ctime TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
