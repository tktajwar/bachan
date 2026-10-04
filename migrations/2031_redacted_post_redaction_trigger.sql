CREATE OR REPLACE FUNCTION update_post_redaction_status()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN

        UPDATE Post
        SET is_redacted = TRUE
        WHERE id = NEW.post;

        RETURN NEW;

    ELSIF TG_OP = 'DELETE' THEN

        UPDATE Post
        SET is_redacted = FALSE
        WHERE id = OLD.post;

        RETURN OLD;

    END IF;

    RETURN NULL;
END;
$$;

CREATE TRIGGER redactedpost_update_redacted
AFTER INSERT OR DELETE ON RedactedPost
FOR EACH ROW
EXECUTE FUNCTION update_post_redaction_status();
