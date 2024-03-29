use anyhow::Result;
use libnss::group::{Group, GroupHooks};
use libnss::interop::Response;
use rusqlite::OpenFlags;
use rusqlite::{params, Connection, Row, NO_PARAMS};

use crate::db::from_result;

// TODO: Find a better way to do this >.<
pub const DEFAULT_PATH: &str = "/etc/group.db";
lazy_static! {
    pub static ref PATH: String = if cfg!(feature = "dynamic_paths") {
        std::env::var("NSS_GROUP_PATH").unwrap_or_else(|_| String::from(DEFAULT_PATH))
    } else {
        String::from(DEFAULT_PATH)
    };
}

pub struct SqliteGroup;
libnss_group_hooks!(sqlite, SqliteGroup);

impl GroupHooks for SqliteGroup {
    fn get_all_entries() -> Response<Vec<Group>> {
        let entries = Connection::open_with_flags(&PATH as &str, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(Into::into)
            .and_then(get_all_entries);

        from_result(entries)
    }

    fn get_entry_by_gid(gid: libc::uid_t) -> Response<Group> {
        let entry = Connection::open_with_flags(&PATH as &str, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(Into::into)
            .and_then(|conn| get_entry_by_gid(conn, gid));

        from_result(entry)
    }

    fn get_entry_by_name(name: String) -> Response<Group> {
        let entry = Connection::open_with_flags(&PATH as &str, OpenFlags::SQLITE_OPEN_READ_ONLY)
            .map_err(Into::into)
            .and_then(|conn| get_entry_by_name(conn, &name));

        from_result(entry)
    }
}

fn get_all_entries(conn: Connection) -> Result<Vec<Group>> {
    conn.prepare(
        "
        SELECT groupname, password, gid, users
        FROM groups
        ",
    )?
    .query_and_then(NO_PARAMS, from_row)?
    .collect()
}
fn get_entry_by_gid(conn: Connection, gid: u32) -> Result<Group> {
    conn.query_row_and_then(
        "
        SELECT groupname, password, gid, users
        FROM groups
        WHERE gid = ?1
        ",
        params![gid],
        from_row,
    )
}
fn get_entry_by_name(conn: Connection, name: &str) -> Result<Group> {
    conn.query_row_and_then(
        "
        SELECT groupname, password, gid, users
        FROM groups
        WHERE groupname = ?1
        ",
        params![name],
        from_row,
    )
}

fn from_row(row: &Row) -> Result<Group> {
    let raw_members: String = row.get(3)?;

    Ok(Group {
        name: row.get(0)?,
        passwd: row.get(1)?,
        gid: row.get(2)?,
        members: raw_members.split(',').map(String::from).collect(),
    })
}
