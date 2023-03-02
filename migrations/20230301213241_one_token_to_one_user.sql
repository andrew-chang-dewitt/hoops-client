-- make users:token 1:1
ALTER TABLE user ADD COLUMN token_id TEXT;
-- no longer need many:many table previously used
DROP TABLE IF EXISTS user_token;
