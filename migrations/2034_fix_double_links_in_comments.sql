UPDATE Post
SET
  comment = regexp_replace(comment, '\[(.+?)\]\s*\[\1\]', '[\1]', 'g');
