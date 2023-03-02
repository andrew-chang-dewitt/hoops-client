CREATE TABLE user (
    user_id UUID PRIMARY KEY NOT NULL,
    handle TEXT NOT NULL
);

CREATE TABLE token (
    access_token TEXT NOT NULL PRIMARY KEY,
    token_type TEXT NOT NULL
);

CREATE TABLE user_token (
    user_id UUID NOT NULL,
    token_id TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES user(user_id),
    FOREIGN KEY(token_id) REFERENCES token(access_token)
);
