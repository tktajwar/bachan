INSERT INTO RedactedPost (
    post, moderator, reason, redacted_time
) SELECT thread_or_reply_id, mod_id, reason, rtime
FROM redacted;
