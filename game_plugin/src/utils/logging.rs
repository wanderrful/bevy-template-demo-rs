/// Provide standardized logging structure without any dependencies.
/// TODO | Get rid of Chrono because I'm only using like 1 or 2 things from it.
#[allow(unused_variables, dead_code)]
use chrono::{SecondsFormat, Utc};

/// Use this to log a message to stdout for any type, at any point
pub trait Loggable {
    /// Log as DEBUG
    fn log_debug(&mut self, message: &str);

    /// Log as INFO
    fn log_info(&mut self, message: &str);

    /// Log as WARN
    fn log_warn(&mut self, message: &str);

    /// Log as ERROR
    fn log_error(&mut self, message: &str);

    /// Get the name of this current type
    fn get_type_name() -> &'static str;
}

impl<T> Loggable for T {
    fn log_debug(&mut self, message: &str) {
        log(_DEBUG, Self::get_type_name(), message);
    }

    fn log_info(&mut self, message: &str) {
        log(_INFO, Self::get_type_name(), message);
    }

    fn log_warn(&mut self, message: &str) {
        log(_WARN, Self::get_type_name(), message);
    }

    fn log_error(&mut self, message: &str) {
        log(_ERROR, Self::get_type_name(), message);
    }

    fn get_type_name() -> &'static str {
        std::any::type_name::<T>()
    }
}

fn get_timestamp() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false)
}

fn log(log_level: &str, type_name: &str, message: &str) {
    println!("{} [{}] {}\t- {} - {}",
             _PREFIX,
             get_timestamp(),
             log_level,
             type_name,
             message);
}

const _PREFIX: &str = "***";
const _DEBUG: &str = "DEBUG";
const _INFO: &str = "INFO";
const _WARN: &str = "WARN";
const _ERROR: &str = "ERROR";
