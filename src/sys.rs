use std::env;

pub fn args() -> Vec<String> {
    env::args().collect()
}
