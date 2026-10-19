DROP INDEX idx_board_threads;

CREATE INDEX idx_board_threads
ON Post (board, bump_time DESC)
WHERE board IS NOT NULL AND is_redacted IS FALSE;
