use std::fs::metadata;
use std::time::UNIX_EPOCH;

pub(crate) fn stat(arguments: &[String]) {
    for argument in arguments.iter() {
        let stats = metadata(argument).unwrap();
        println!("File: {}", argument);
        println!("Size: {}", stats.len());
        print!("Type: ");
        if stats.is_file() {
            println!("File");
        }
        else if stats.is_dir() {
            println!("Directory");
        }
        else if stats.is_symlink() {
            println!("Symbolic Link");
        }
        if stats.permissions().readonly() {
            println!("Permission: Read-only");
        }
        println!("Acces: {} seconds since UNIX Epoch", stats.accessed().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs());
        println!("Modified: {} seconds since UNIX Epoch", stats.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs());
        println!("Created: {} seconds since UNIX Epoch", stats.created().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs());
    }
}