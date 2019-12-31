use libnss::interop::Response;
use libnss::shadow::{Shadow, ShadowHooks};

use rusqlite::OpenFlags;
use rusqlite::{params, Connection, Result, Row, NO_PARAMS};

use crate::db::from_result;

pub const PATH: &str = "/etc/shadow.db";

pub struct SqliteShadow;
libnss_shadow_hooks!(sqlite, SqliteShadow);

impl ShadowHooks for SqliteShadow {
    fn get_all_entries() -> Response<Vec<Shadow>> {
        let entries = Connection::open_with_flags(PATH, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .and_then(get_all_entries);

        from_result(entries)
    }

    fn get_entry_by_name(name: String) -> Response<Shadow> {
        let entry = Connection::open_with_flags(PATH, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .and_then(|conn| get_entry_by_name(conn, &name));

        from_result(entry)
    }
}

fn get_all_entries(conn: Connection) -> Result<Vec<Shadow>> {
    conn.prepare(
        "
        SELECT username, encrypted_password, date_of_last_password_change, minimum_password_age, maximum_password_age, password_warning_period, account_expiration_date
        FROM shadow
        ",
    )?
    .query_and_then(NO_PARAMS, from_row)?
    .collect()
}
fn get_entry_by_name(conn: Connection, name: &str) -> Result<Shadow> {
    conn.query_row_and_then(
        "
        SELECT username, encrypted_password, date_of_last_password_change, minimum_password_age, maximum_password_age, password_warning_period, account_expiration_date
        FROM shadow
        WHERE username = ?1
        ",
        params![name],
        from_row,
    )
}

fn from_row(row: &Row) -> Result<Shadow> {
    Ok(Shadow {
        name: row.get(0)?,
        passwd: row.get(1)?,
        last_change: row.get(2)?,
        change_min_days: row.get(3)?,
        change_max_days: row.get(4)?,
        change_warn_days: row.get(5)?,
        change_inactive_days: row.get(6)?,
        expire_date: row.get(7)?,
        // TODO: figure out how to handle reserved.
        // Ideas:
        //  * blob <-> u64:
        reserved: 0,
    })
}
