UPDATE Post
SET
  comment = regexp_replace(comment, '<span class="quote">>>([0-9a-fA-F]+)</span>', '>>\1', 'g');
