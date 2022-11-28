USE movienight;

ALTER TABLE users
    DROP COLUMN token,
    DROP COLUMN token_expiration;