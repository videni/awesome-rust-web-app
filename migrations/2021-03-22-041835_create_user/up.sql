CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Must double quote user as it is a PostgreSQL keyword
-- Though it is not convenient, I'd like to keep table name singulear form.
CREATE TABLE "user" (
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username varchar(40) NOT NULL,
    password varchar(300) NOT NULL,
    created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
    email varchar(128) DEFAULT NULL,
    failures_num smallint DEFAULT '0',
    first_failed_at timestamp NULL DEFAULT NULL,
    lock_expires_at timestamp NULL DEFAULT NULL,
    enabled boolean NOT NULL DEFAULT TRUE,
    salt varchar(255) DEFAULT NULL,
    UNIQUE(username)
);

comment on column "user".user_id is 'User ID';
comment on column "user".username is 'User Login';
comment on column "user".password is 'User Password';
comment on column "user".created_at is 'User Created Time';
comment on column "user".updated_at is 'User Updated Time';
comment on column "user".email is 'User Email';
comment on column "user".failures_num is 'Failure Number';
comment on column "user".first_failed_at is 'First Failure';
comment on column "user".lock_expires_at is 'Expiration Lock Dates';
comment on column "user".enabled is 'User Is enabled';
comment on column "user".salt is 'Salt to encode password';
