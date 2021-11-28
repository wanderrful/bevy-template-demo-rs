/// Provide standardized logging structure without any dependencies.
#[allow(unused_variables, dead_code)]
use chrono::{SecondsFormat, Utc};

/// Use this to log a message to stdout for any type, at any point
pub trait Loggable {
    /// Log as DEBUG
    fn debug(&mut self, message: &str);

    /// Log as INFO
    fn info(&mut self, message: &str);

    /// Log as WARN
    fn warn(&mut self, message: &str);

    /// Log as ERROR
    fn error(&mut self, message: &str);

    /// Get the name of this current type
    fn get_type_name() -> &'static str;
}

impl<T> Loggable for T {
    fn debug(&mut self, message: &str) {
        println!("{} [{}] {}\t- {} - {}",
                 _PREFIX,
                 get_timestamp(),
                 _DEBUG,
                 Self::get_type_name(),
                 message);
    }

    fn info(&mut self, message: &str) {
        println!("{} [{}] {}\t- {} - {}",
                 _PREFIX,
                 get_timestamp(),
                 _INFO,
                 Self::get_type_name(),
                 message);
    }

    fn warn(&mut self, message: &str) {
        println!("{} [{}] {}\t- {} - {}",
                 _PREFIX,
                 get_timestamp(),
                 _WARN,
                 Self::get_type_name(),
                 message);
    }

    fn error(&mut self, message: &str) {
        println!("{} [{}] {}\t- {} - {}",
                 _PREFIX,
                 get_timestamp(),
                 _ERROR,
                 Self::get_type_name(),
                 message);
    }

    fn get_type_name() -> &'static str {
        std::any::type_name::<T>()
    }
}

fn get_timestamp() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false)
}

const _PREFIX: &str = "***";
const _DEBUG: &str = "DEBUG";
const _INFO: &str = "INFO";
const _WARN: &str = "WARN";
const _ERROR: &str = "ERROR";
