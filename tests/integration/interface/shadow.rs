use std::sync;

use libnss::interop::Response;
use libnss::shadow::ShadowHooks;
use nss_sqlite::shadow::SqliteShadow;

use crate::common;

fn setup() {
    static INIT: sync::Once = sync::Once::new();
    common::setup(|conn| {
        INIT.call_once(|| {
            conn.execute_batch(
                r#"
                INSERT INTO shadow ("username") VALUES ("test-user");
                "#,
            )
            .expect("failed to create test shadow users")
        })
    })
}

#[test]
fn get_all_shadows() {
    setup();

    let entries = SqliteShadow::get_all_entries();
    if let Response::Success(shadows) = entries {
        assert_eq!(1, shadows.len());
    } else {
        panic!("unexpected response type");
    }
}

#[test]
fn get_existing_user() {
    setup();

    let entries = SqliteShadow::get_entry_by_name("test-user".to_string());
    if let Response::Success(shadow) = entries {
        assert_eq!("test-user", shadow.name);
    } else if let Response::NotFound = entries {
        panic!("should've returned a row")
    } else {
        panic!("unexpected response type")
    }
}
