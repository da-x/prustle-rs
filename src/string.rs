extern crate easy_strings;

pub use self::easy_strings::{EZString, ez};

// Example of some obsolete functions

pub fn upper(s : &EZString) -> EZString {
    s.to_uppercase()
}

pub fn lower(s : &EZString) -> EZString {
    s.to_lowercase()
}

// Some global builtins that are not locale-dependent:

#[allow(non_upper_case_globals)]
pub static ascii_letter: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[allow(non_upper_case_globals)]
pub static ascii_lowercase: &'static str = "abcdefghijklmnopqrstuvwxyz";

#[allow(non_upper_case_globals)]
pub static ascii_upppecase: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

#[allow(non_upper_case_globals)]
pub static digits: &'static str = "0123456789";

#[allow(non_upper_case_globals)]
pub static hexdigits: &'static str = "0123456789abcdefABCDEF";

#[allow(non_upper_case_globals)]
pub static octdigits: &'static str = "01234567";
