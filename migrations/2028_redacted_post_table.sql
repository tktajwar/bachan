CREATE TABLE RedactedPost (
       post BIGINT NOT NULL REFERENCES Post (id) ON DELETE CASCADE PRIMARY KEY,
       moderator INT NOT NULL REFERENCES Mod (id) ON DELETE CASCADE,
       reason TEXT NOT NULL,
       redacted_time TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

       CONSTRAINT chk_redacted_reason_integrity
        CHECK (
    	    reason IS NOT NULL AND TRIM(reason) != ''
	)
);
