CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE ModToken (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       hash VARCHAR NOT NULL
);
