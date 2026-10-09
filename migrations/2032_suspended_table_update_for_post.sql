DELETE FROM Suspended;

ALTER TABLE Suspended
DROP COLUMN thread_or_reply_id;

ALTER TABLE Suspended
ADD COLUMN post BIGINT REFERENCES Post(id) ON DELETE CASCADE;

CREATE INDEX idx_suspended
ON Suspended (uid, until DESC);
