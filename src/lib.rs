use chrono::Local;
use colored::Colorize;

pub struct ConsoleLogger {
    format: String,
}

impl ConsoleLogger {
    pub fn new(format: &str) -> ConsoleLogger {
        ConsoleLogger {
            format: String::from(format),
        }
    }

    fn parser(&self, message: &str, caller: &str) -> String {
        let mut parsed_string = String::from(&self.format);

        if self.format.contains("%dt") {
            let datetime_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            parsed_string = parsed_string.replace("%dt", &datetime_now);
        }

        if self.format.contains("%d") {
            let date_now = Local::now().format("%Y-%m-%d").to_string();
            parsed_string = parsed_string.replace("%d", &date_now);
        }

        if self.format.contains("%m") {
            parsed_string = parsed_string.replace("%m", message);
        }

        if self.format.contains("%i") {
            parsed_string = parsed_string.replace("%i", caller);
        }

        parsed_string
    }

    pub fn warn(&self, message: &str) {
        println!("{}", self.parser(message, "Warn").yellow());
    }

    pub fn log(&self, message: &str) {
        println!("{}", self.parser(message, "Log").green());
    }

    pub fn info(&self, message: &str) {
        println!("{}", self.parser(message, "Info").blue());
    }

    pub fn debug(&self, message: &str) {
        println!("{}", self.parser(message, "Debug").purple());
    }

    pub fn error(&self, message: &str) {
        println!("{}", self.parser(message, "Error").red());
    }
}
