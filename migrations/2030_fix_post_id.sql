SELECT setval(
  pg_get_serial_sequence('Post', 'id'),
  COALESCE((SELECT MAX(id) FROM Post), 1),
  true
);

ALTER TABLE Post
ALTER COLUMN id SET GENERATED ALWAYS;
