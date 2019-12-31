CREATE TABLE IF NOT EXISTS "groups" (
    "groupname" TEXT    NOT NULL,
    "password"  TEXT    NOT NULL,
    "gid"       INTEGER NOT NULL,
    "users"     TEXT    NOT NULL
);

CREATE INDEX IF NOT EXISTS "group_groupname"  ON groups(groupname);
CREATE INDEX IF NOT EXISTS "group_gid"        ON groups(gid);
