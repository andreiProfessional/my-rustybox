use std::fs::create_dir;

pub(crate) fn mkdir(arguments: &[String]) {
    for argument in arguments.iter() {
        create_dir(argument).unwrap();
    }
}