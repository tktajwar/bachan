ALTER TABLE thread ADD COLUMN IF NOT EXISTS
reply_count INT DEFAULT 0 NOT NULL;

CREATE OR REPLACE FUNCTION update_thread_reply_count()
RETURNS TRIGGER AS $$
BEGIN
	UPDATE Thread
	SET reply_count = reply_count + 1
	WHERE id = NEW.tid;
	RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER reply_insert_trigger_rc
AFTER INSERT ON Reply
FOR EACH ROW
EXECUTE FUNCTION update_thread_reply_count();
