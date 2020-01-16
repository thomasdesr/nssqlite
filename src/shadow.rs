use libnss::interop::Response;
use libnss::shadow::{Shadow, ShadowHooks};

use rusqlite::OpenFlags;
use rusqlite::{params, Connection, Result, Row, NO_PARAMS};

use crate::db::from_result;

// TODO: Find a better way to do this >.<
pub const DEFAULT_PATH: &str = "/etc/shadow.db";
lazy_static! {
    pub static ref PATH: String = if cfg!(feature = "dynamic_paths") {
        std::env::var("NSS_SHADOW_PATH").unwrap_or_else(|_| String::from(DEFAULT_PATH))
    } else {
        String::from(DEFAULT_PATH)
    };
}

pub struct SqliteShadow;
libnss_shadow_hooks!(sqlite, SqliteShadow);

impl ShadowHooks for SqliteShadow {
    fn get_all_entries() -> Response<Vec<Shadow>> {
        let entries = Connection::open_with_flags(&PATH as &str, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .and_then(get_all_entries);

        from_result(entries)
    }

    fn get_entry_by_name(name: String) -> Response<Shadow> {
        let entry = Connection::open_with_flags(&PATH as &str, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .and_then(|conn| get_entry_by_name(conn, &name));

        from_result(entry)
    }
}

fn get_all_entries(conn: Connection) -> Result<Vec<Shadow>> {
    conn.prepare(
        "
        SELECT username, encrypted_password, date_of_last_password_change, minimum_password_age, maximum_password_age, password_warning_period, password_inactivity_period, account_expiration_date, reserved_field
        FROM shadow
        ",
    )?
    .query_and_then(NO_PARAMS, from_row)?
    .collect()
}
fn get_entry_by_name(conn: Connection, name: &str) -> Result<Shadow> {
    conn.query_row_and_then(
        "
        SELECT username, encrypted_password, date_of_last_password_change, minimum_password_age, maximum_password_age, password_warning_period, password_inactivity_period, account_expiration_date, reserved_field
        FROM shadow
        WHERE username = ?1
        ",
        params![name],
        from_row,
    )
}

fn from_row(row: &Row) -> Result<Shadow> {
    // Default values for each of these are based on `glibc/shadow/sgetspent_r.c`

    Ok(Shadow {
        name: row.get(0)?,
        passwd: row.get(1)?,
        last_change: row.get::<usize, Option<i64>>(2)?.unwrap_or(-1),
        change_min_days: row.get::<usize, Option<i64>>(3)?.unwrap_or(-1),
        change_max_days: row.get::<usize, Option<i64>>(4)?.unwrap_or(-1),
        change_warn_days: row.get::<usize, Option<i64>>(5)?.unwrap_or(-1),
        change_inactive_days: row.get::<usize, Option<i64>>(6)?.unwrap_or(-1),
        expire_date: row.get::<usize, Option<i64>>(7)?.unwrap_or(-1),
        reserved: row.get::<usize, Option<u32>>(8)?.unwrap_or(!0u32) as u64,
    })
}
