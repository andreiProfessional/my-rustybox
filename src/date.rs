use chrono::prelude::*;

pub(crate) fn date(arguments: &[String]) {
    if let Some(arg) = arguments.get(0) {
        match arg.as_str() {
            "-u" | "--utc" => {
                let time = Utc::now();
                println!("{}", time.format("%Y-%m-%d %H:%M:%S"));
            }
            _ => {
                let time = Local::now();
                println!("{}", time.format("%Y-%m-%d %H:%M:%S"));
            }
        }
    } else {
        let time = Local::now();
        println!("{}", time.format("%Y-%m-%d %H:%M:%S"));
    }
}
