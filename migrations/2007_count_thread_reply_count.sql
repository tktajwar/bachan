UPDATE thread t
SET reply_count = (
    SELECT COUNT(1) FROM reply r
    WHERE r.tid = t.id
);
