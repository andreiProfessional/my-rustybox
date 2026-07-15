use std::{thread, time::Duration};

pub(crate) fn sleep(arguments: &[String]) {
    match arguments[0].as_str() {
        "-ms" | "--miliseconds" => {
            let mut sum: u64 = 0;
            for argument in arguments[1..].iter() {
                sum += argument.parse::<u64>().unwrap();
            }
            thread::sleep(Duration::from_millis(sum))
        },
        "-s" | "--seconds" => {
            let mut sum: f64 = 0.0;
            for argument in arguments[1..].iter() {
                sum += argument.parse::<f64>().unwrap();
            }
            thread::sleep(Duration::from_secs_f64(sum))
        },
        "-m" | "--minutes" => {
            let mut sum: f64 = 0.0;
            for argument in arguments[1..].iter() {
                sum += argument.parse::<f64>().unwrap();
            }
            thread::sleep(Duration::from_secs_f64(60.0 * sum))
        },
        _ => {
            let mut sum: f64 = 0.0;
            for argument in arguments[0..].iter() {
                sum += argument.parse::<f64>().unwrap();
            }
            thread::sleep(Duration::from_secs_f64(sum))
        }
    }
}