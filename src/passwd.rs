use libnss::interop::Response;
use libnss::passwd::{Passwd, PasswdHooks};

use rusqlite::OpenFlags;
use rusqlite::{params, Connection, Result, Row, NO_PARAMS};

use crate::db::from_result;

// TODO: Find a better way to do this >.<
pub const DEFAULT_PATH: &str = "/etc/passwd.db";
lazy_static! {
    pub static ref PATH: String = if cfg!(feature = "dynamic_paths") {
        std::env::var("NSS_PASSWD_PATH").unwrap_or_else(|_| String::from(DEFAULT_PATH))
    } else {
        String::from(DEFAULT_PATH)
    };
}

pub struct SqlitePasswd;
libnss_passwd_hooks!(sqlite, SqlitePasswd);

impl PasswdHooks for SqlitePasswd {
    fn get_all_entries() -> Response<Vec<Passwd>> {
        let entries = Connection::open_with_flags(&PATH as &str, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .and_then(get_all_entries);

        from_result(entries)
    }

    fn get_entry_by_uid(uid: libc::uid_t) -> Response<Passwd> {
        let entry = Connection::open_with_flags(&PATH as &str, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .and_then(|conn| get_entry_by_uid(conn, uid));

        from_result(entry)
    }

    fn get_entry_by_name(name: String) -> Response<Passwd> {
        let entry = Connection::open_with_flags(&PATH as &str, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .and_then(|conn| get_entry_by_name(conn, &name));

        from_result(entry)
    }
}

fn get_all_entries(conn: Connection) -> Result<Vec<Passwd>> {
    conn.prepare(
        "
        SELECT username, password, uid, gid, comment, directory, shell
        FROM passwd
        ",
    )?
    .query_and_then(NO_PARAMS, from_row)?
    .collect()
}
fn get_entry_by_uid(conn: Connection, uid: u32) -> Result<Passwd> {
    conn.query_row_and_then(
        "
        SELECT username, password, uid, gid, comment, directory, shell
        FROM passwd
        WHERE uid = ?1
        ",
        params![uid],
        from_row,
    )
}
fn get_entry_by_name(conn: Connection, name: &str) -> Result<Passwd> {
    conn.query_row_and_then(
        "
        SELECT username, password, uid, gid, comment, directory, shell
        FROM passwd
        WHERE username = ?1
        ",
        params![name],
        from_row,
    )
}

fn from_row(row: &Row) -> Result<Passwd> {
    Ok(Passwd {
        name: row.get(0)?,
        passwd: row.get(1)?,
        uid: row.get(2)?,
        gid: row.get(3)?,
        gecos: row.get(4)?,
        dir: row.get(5)?,
        shell: row.get(6)?,
    })
}
