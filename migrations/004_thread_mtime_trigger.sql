CREATE OR REPLACE FUNCTION update_thread_mtime()
RETURNS TRIGGER AS $$
BEGIN
	UPDATE Thread
	SET mtime = NOW()
	WHERE id = NEW.tid;
	RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER reply_insert_trigger
AFTER INSERT ON Reply
FOR EACH ROW
EXECUTE FUNCTION update_thread_mtime();
