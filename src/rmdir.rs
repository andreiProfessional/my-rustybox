use std::fs::remove_dir;

pub(crate) fn rmdir(arguments: &[String]) {
    for argument in arguments.iter() {
        remove_dir(argument).unwrap();
    }
}