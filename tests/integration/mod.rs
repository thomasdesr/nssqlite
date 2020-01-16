mod common;

mod interface {
    mod shadow;
}

#[cfg(target_os = "linux")]
mod libc {
    mod groups;
    mod nss;
    mod passwd;
}
