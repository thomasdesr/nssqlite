CREATE TABLE IF NOT EXISTS "passwd" (
    "username"  TEXT    NOT NULL,
    "password"  TEXT    NOT NULL DEFAULT "*",
    "uid"       INTEGER NOT NULL,
    "gid"       INTEGER NOT NULL,
    "comment"   TEXT    NOT NULL,
    "directory" TEXT    NOT NULL,
    "shell"     TEXT    NOT NULL
);

CREATE INDEX IF NOT EXISTS "passwd_username"  ON passwd(username);
CREATE INDEX IF NOT EXISTS "passwd_uid"       ON passwd(uid);
