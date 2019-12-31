use libnss::interop::Response;

use rusqlite::Error::QueryReturnedNoRows;
use rusqlite::Result;

pub fn from_result<T>(res: Result<T>) -> Response<T> {
    match res {
        Ok(r) => Response::Success(r),
        Err(QueryReturnedNoRows) => Response::NotFound,
        _ => Response::Unavail,
    }
}
