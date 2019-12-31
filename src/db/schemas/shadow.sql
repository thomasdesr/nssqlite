CREATE TABLE "shadow" IF NOT EXISTS (
    "username"                      TEXT    NOT NULL,
    "encrypted_password"            TEXT    NOT NULL DEFAULT "*",
    "date_of_last_password_change"  INTEGER NOT NULL,
    "minimum_password_age"          INTEGER NOT NULL,
    "maximum_password_age"          INTEGER NOT NULL,
    "password_warning_period"       INTEGER NOT NULL,
    "password_inactivity_period"    INTEGER NOT NULL,
    "account_expiration_date"       INTEGER NOT NULL,
    "reserved_field"                BLOB    NOT NULL
);

CREATE INDEX IF NOT EXISTS  "shadow_username"  ON shadow(username);