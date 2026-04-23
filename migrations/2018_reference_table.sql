CREATE TABLE reference (
       id SERIAL PRIMARY KEY,
       referencer INT NOT NULL,
       referencee INT NOT NULL
);

CREATE INDEX reference_referencee
ON reference(referencee);
