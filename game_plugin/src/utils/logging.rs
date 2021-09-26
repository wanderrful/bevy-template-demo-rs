/// Provide standardized logging structure without any dependencies.
pub struct Logger<'a> {
    class: &'a str
}

#[allow(dead_code, unused_variables)]
impl Logger<'_> {

    pub const fn new(class: &str) -> Logger {
        Logger {
            class: class
        }
    }

    pub fn debug(self, message: &str) {
        println!("*** {} {} - {}", _DEBUG, self.class, message);
    }

    pub fn info(self, message: &str) {
        println!("*** {} {} - {}", _INFO, self.class, message);
    }

    pub fn warn(self, message: &str) {
        println!("*** {} {} - {}", _WARN, self.class, message);
    }

    pub fn error(self, message: &str) {
        println!("*** {} {} - {}", _ERROR, self.class, message);
    }
}

const _DEBUG: &str = "DEBUG";
const _INFO: &str = "INFO";
const _WARN: &str = "WARN";
const _ERROR: &str = "ERROR";
