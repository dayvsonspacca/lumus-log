use chrono::Local;
use colored::Colorize;

struct ConsoleLogger {
    verbose: bool,
}

impl ConsoleLogger {
    pub fn new(verbose: bool) -> ConsoleLogger {
        ConsoleLogger { verbose }
    }

    pub fn warn(&self, message: &str) {
        if self.verbose {
            let date_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            println!(
                "{}",
                format!("[ {} - Warning ] - {}", date_now, message).yellow()
            );
        } else {
            println!("{}", format!("[ Warning ] - {}", message).yellow())
        }
    }

    pub fn info(&self, message: &str) {
        if self.verbose {
            let date_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            println!(
                "{}",
                format!("[ {} - Info ] - {}", date_now, message).cyan()
            );
        } else {
            println!("{}", format!("[ Info ] - {}", message).cyan())
        }
    }

    pub fn error(&self, message: &str) {
        if self.verbose {
            let date_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            println!(
                "{}",
                format!("[ {} - Error ] - {}", date_now, message).red()
            );
        } else {
            println!("{}", format!("[ Error ] - {}", message).red())
        }
    }

    pub fn log(&self, message: &str) {
        if self.verbose {
            let date_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            println!(
                "{}",
                format!("[ {} - Log ] - {}", date_now, message).green()
            );
        } else {
            println!("{}", format!("[ Log ] - {}", message).green())
        }
    }

    pub fn debug(&self, message: &str) {
        if self.verbose {
            let date_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

            println!(
                "{}",
                format!("[ {} - Debug ] - {}", date_now, message).purple()
            );
        } else {
            println!("{}", format!("[ Debug ] - {}", message).purple())
        }
    }
}
