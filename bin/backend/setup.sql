CREATE TABLE "test"."users" (
    id uuid PRIMARY KEY,
    username VARCHAR NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    modified_at TIMESTAMP WITH TIME ZONE NOT NULL
);