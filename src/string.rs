extern crate easy_strings;

pub use self::easy_strings::{EZString, ez};

// Example of some obsolete functions

pub fn upper(s : &EZString) -> EZString {
    s.to_uppercase()
}

pub fn lower(s : &EZString) -> EZString {
    s.to_lowercase()
}
