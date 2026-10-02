DELETE FROM Reply
WHERE tid IN (
      SELECT ID FROM Thread
      WHERE board IN ('g', 'kd', 'gh', 'r')
);

DELETE FROM Thread
WHERE board IN ('g', 'kd', 'gh', 'r');

DELETE FROM Board
WHERE url IN ('g', 'kd', 'gh', 'r');
