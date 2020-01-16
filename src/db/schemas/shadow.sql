-- Types based on `glibc/shadow/shadow.h`
-- Nullability in this type was defined based on `glibc/shadow/sgetspent_r.c`

CREATE TABLE IF NOT EXISTS "shadow"  (
    "username"                      TEXT    NOT NULL,
    "encrypted_password"            TEXT    NOT NULL DEFAULT "*",
    "date_of_last_password_change"  INTEGER,
    "minimum_password_age"          INTEGER,
    "maximum_password_age"          INTEGER,
    "password_warning_period"       INTEGER,
    "password_inactivity_period"    INTEGER,
    "account_expiration_date"       INTEGER,
    "reserved_field"                INTEGER -- Should be an u32, but that'll fit in i64
);

CREATE INDEX IF NOT EXISTS  "shadow_username"  ON shadow(username);
