use std::fs::{remove_file, remove_dir, remove_dir_all};

pub(crate) fn rm(arguments: &[String]) {
    match arguments[0].as_str() {
        "-d" | "--dir" => {
            for file in arguments[1..].iter() {
                remove_dir_all(file).unwrap();
            }
        },
        "-r" | "--recursive" => {
            for file in arguments[1..].iter() {
                remove_dir_all(file).unwrap();
            }
        },
        _ => {
            for file in arguments.iter() {
                remove_file(file).unwrap();
            }
        }
    }
}