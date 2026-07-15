use std::fs::rename;

pub(crate) fn mv(arguments: &[String]) {
    rename(arguments[0].as_str(), arguments[1].as_str()).unwrap()
}