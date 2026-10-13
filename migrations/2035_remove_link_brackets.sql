UPDATE Post
SET
  comment = regexp_replace(comment, '\[(https:\S+?)\]', '\1', 'g');
