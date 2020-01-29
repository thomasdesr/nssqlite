use anyhow::Result;
use libnss::interop::Response;

pub fn from_result<T>(res: Result<T>) -> Response<T> {
    res.map(Response::Success)
        .unwrap_or_else(|err| match err.downcast::<rusqlite::Error>() {
            Ok(rusqlite::Error::QueryReturnedNoRows) => Response::NotFound,
            _ => Response::Unavail,
        })
}
