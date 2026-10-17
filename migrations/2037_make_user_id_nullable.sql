ALTER TABLE Post ALTER COLUMN user_id DROP NOT NULL;

UPDATE Post
SET
  user_id = NULL
WHERE user_id = '0.0.0.0'::inet;
