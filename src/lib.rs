use std::{fs::File, io::Write};

use chrono::Local;
use colored::{ColoredString, Colorize};

pub struct ConsoleLogger {
    format: String,
}

impl ConsoleLogger {
    /// Creates a new `ConsoleLogger` instance with the given log format.
    /// # Example
    /// ```
    /// use lumus_logger::ConsoleLogger;
    ///
    /// let logger = ConsoleLogger::new("[ %dt - %i ] - %m");
    /// logger.warn("Warn message");
    /// logger.info("Info message");
    /// logger.log("Log message");
    /// logger.debug("Debug message");
    /// logger.error("Error message");
    /// ```
    pub fn new(format: &str) -> ConsoleLogger {
        ConsoleLogger {
            format: String::from(format),
        }
    }

    /// Logger parser, transform the message in the format specified in new.
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

    /// Use `println!()` to output a yellow message in `%m` format str.
    ///- Return `ColoredString`
    pub fn warn(&self, message: &str) -> ColoredString {
        let warn = self.parser(message, "Warn").yellow();
        println!("{}", warn);
        warn
    }

    /// Use `println!()` to output a green message in `%m` format str.
    ///- Return `ColoredString`
    pub fn log(&self, message: &str) -> ColoredString {
        let log = self.parser(message, "Log").green();
        println!("{}", log);
        log
    }

    /// Use `println!()` to output a blue message in `%m` format str.
    ///- Return `ColoredString`
    pub fn info(&self, message: &str) -> ColoredString {
        let info = self.parser(message, "Info").blue();
        println!("{}", info);
        info
    }

    /// Use `println!()` to output a purple message in `%m` format str.
    ///- Return `ColoredString`
    pub fn debug(&self, message: &str) -> ColoredString {
        let debug = self.parser(message, "Debug").purple();
        println!("{}", debug);
        debug
    }

    /// Use `println!()` to output a red message in `%m` format str.
    ///- Return `ColoredString`
    pub fn error(&self, message: &str) -> ColoredString {
        let error = self.parser(message, "Error").red();
        println!("{}", error);
        error
    }
}

pub struct FileLogger {
    format: String,
    file: File,
}

impl FileLogger {
    /// Creates a new `FileLogger` instance with the given log format, and file name.
    /// # Example
    /// ```
    /// use lumus_logger::FileLogger;
    ///
    /// let mut logger = FileLogger::new("[ %dt - %i ] - %m", "log.txt");
    /// logger.warn("Warn message");
    /// logger.info("Info message");
    /// logger.log("Log message");
    /// logger.debug("Debug message");
    /// logger.error("Error message");
    /// ```
    pub fn new(format: &str, name: &str) -> FileLogger {
        match File::open(name) {
            Ok(_) => {
                panic!("{} file already exists!", name);
            }
            Err(_) => {}
        }

        let file = match File::create(name) {
            Ok(file) => file,
            Err(e) => {
                panic!("Failed to generate {} file: {}", name, e);
            }
        };

        FileLogger {
            format: String::from(format),
            file,
        }
    }

    /// Logger parser, transform the message in the format specified in new.
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

        parsed_string.push_str("\n");
        parsed_string
    }

    /// Use `File::write_all()` to write a WARN log `as_bytes` of the `%m` message.
    ///- Panic if `File` already exists.
    pub fn warn(&mut self, message: &str) {
        let warn = self.parser(message, "Warn");
        match self.file.write_all(warn.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to write WARN log: {}", e);
            }
        }
    }

    /// Use `File::write_all()` to write a LOG log `as_bytes` of the `%m` message.
    ///- Panic if `File` already exists.
    pub fn log(&mut self, message: &str) {
        let warn = self.parser(message, "Log");
        match self.file.write_all(warn.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to write LOG log: {}", e);
            }
        }
    }

    /// Use `File::write_all()` to write a INFO log `as_bytes` of the `%m` message.
    ///- Panic if `File` already exists.
    pub fn info(&mut self, message: &str) {
        let warn = self.parser(message, "Info");
        match self.file.write_all(warn.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to write INFO log: {}", e);
            }
        }
    }

    /// Use `File::write_all()` to write a DEBUG log `as_bytes` of the `%m` message.
    ///- Panic if `File` already exists.
    pub fn debug(&mut self, message: &str) {
        let warn = self.parser(message, "Debug");
        match self.file.write_all(warn.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to write DEBUG log: {}", e);
            }
        }
    }

    /// Use `File::write_all()` to write a ERROR log `as_bytes` of the `%m` message.
    ///- Panic if `File` already exists.
    pub fn error(&mut self, message: &str) {
        let warn = self.parser(message, "Error");
        match self.file.write_all(warn.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Failed to write ERROR log: {}", e);
            }
        }
    }
}
