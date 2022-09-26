use crate::utils::time::{FRENCH_TIME_FORMAT, local_timestamp_now};

pub struct MiraiLogger;

pub trait MiraiLog {
    fn info(l: String);
    fn error(l: String);
    fn warn(l: String);
    fn debug(l: String);
}

impl MiraiLog for MiraiLogger {
    fn info(l: String) {
        println!("{} INFO - {}", local_timestamp_now().format(FRENCH_TIME_FORMAT), l);
    }

    fn error(l: String) {
        eprintln!("{} ERROR - {}", local_timestamp_now().format(FRENCH_TIME_FORMAT), l);
    }

    fn warn(l: String) {
        eprintln!("{} WARN - {}", local_timestamp_now().format(FRENCH_TIME_FORMAT), l);
    }

    fn debug(l: String) {
        println!("{} DEBUG - {}", local_timestamp_now().format(FRENCH_TIME_FORMAT), l);
    }
}