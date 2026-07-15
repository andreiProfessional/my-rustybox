mod echo;
mod env;
mod pwd;
mod mkdir;
mod rmdir;
mod sleep;
mod stat;
mod cat;
mod date;
mod mv;
mod rm;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();

    let command = match arguments.get(1) {
        None => panic!("No command provided"),
        Some(command) => command,
    };

    match command.as_str() {
        "echo" => echo::echo(&arguments[2..]),
        "env" => env::env(),
        "pwd" => pwd::pwd(),
        "mkdir" => mkdir::mkdir(&arguments[2..]),
        "rmdir" => rmdir::rmdir(&arguments[2..]),
        "sleep" => sleep::sleep(&arguments[2..]),
        "stat" => stat::stat(&arguments[2..]),
        "cat" => cat::cat(&arguments[2..]),
        "date" => date::date(&arguments[2..]),
        "mv" => mv::mv(&arguments[2..]),
        "rm" => rm::rm(&arguments[2..]),
        _ => panic!("Unrecognized command"),
    }
}
