use std::fs::read_to_string;

pub(crate) fn cat(arguments: &[String]) {
    for path in arguments {
        let file = read_to_string(path).unwrap();
        for line in file.lines() {
            println!("{}", line);
        }
    }
}