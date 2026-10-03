CREATE OR REPLACE FUNCTION update_thread_metadata_on_reply()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.thread_id IS NOT NULL THEN

       UPDATE Post
       SET
	reply_count = reply_count + 1,
	is_saged = is_saged OR reply_count > 255,
	change_time = NEW.birth_time,
	bump_time = CASE WHEN is_saged IS FALSE THEN NEW.birth_time ELSE bump_time END
       WHERE id = NEW.thread_id;

    END IF;

    RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_thread_metadata_on_reply_trigger
AFTER INSERT ON Post
FOR EACH ROW
EXECUTE FUNCTION update_thread_metadata_on_reply();
