use std::fs;
use std::result::{Result};
use std::string::{String};
use std::io;

pub use self::fs::create_dir as mkdir;
pub use self::fs::create_dir_all as makedirs;
pub use self::fs::remove_file as remove;
pub use self::fs::remove_dir as rmdir;

#[cfg(any(unix))]
extern crate uname;

#[cfg(any(unix))]
pub fn uname() -> Result<(String, String, String, String, String), io::Error> {
    uname::Info::new().map(|i| {
        (i.sysname, i.nodename, i.release, i.version, i.machine)
    })
}
