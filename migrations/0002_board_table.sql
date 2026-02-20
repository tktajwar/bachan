CREATE TABLE board (
       url VARCHAR PRIMARY KEY,
       label TEXT NOT NULL,
       category VARCHAR NOT NULL REFERENCES BoardCategory(categoryname)
);
