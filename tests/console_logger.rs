use chrono::Local;
use colored::Colorize;
use lumus_logger::ConsoleLogger;

/* 
Warn messages tests
*/
#[test]
fn test_warn_with_datetime_info_message() {
    let logger = ConsoleLogger::new("[ %dt - %i ] - %m");
    
    let warn = logger.warn("Warn message");
    let datetime_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    assert_eq!(warn, format!("[ {} - Warn ] - Warn message", datetime_now).yellow());
}

#[test]
fn test_warn_with_date_info_message() {
    let logger = ConsoleLogger::new("[ %d - %i ] - %m");
    
    let warn = logger.warn("Warn message");
    let date_now = Local::now().format("%Y-%m-%d").to_string();

    assert_eq!(warn, format!("[ {} - Warn ] - Warn message", date_now).yellow());
}

#[test]
fn test_warn_just_message() {
    let logger = ConsoleLogger::new("%m");
    
    let warn = logger.warn("Warn message");

    assert_eq!(warn, "Warn message".yellow());
}

/* 
Info messages tests
*/
#[test]
fn test_info_with_datetime_info_message() {
    let logger = ConsoleLogger::new("(%dt) - (%i) - (%m)");
    
    let info = logger.info("Info message");
    let datetime_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    assert_eq!(info, format!("({}) - (Info) - (Info message)", datetime_now).blue());
}

#[test]
fn test_info_with_date_info_message() {
    let logger = ConsoleLogger::new("|%d | | %i | - |%m|");
    
    let info = logger.info("Info message");
    let date_now = Local::now().format("%Y-%m-%d").to_string();

    assert_eq!(info, format!("|{} | | Info | - |Info message|", date_now).blue());
}

#[test]
fn test_info_just_message() {
    let logger = ConsoleLogger::new("'%m'");
    
    let info = logger.info("Info message");

    assert_eq!(info, "'Info message'".blue());
}

/* 
Log messages tests
*/
#[test]
fn test_log_with_datetime_message() {
    let logger = ConsoleLogger::new("%dt;%m");
    
    let log = logger.log("Log message");
    let datetime_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    assert_eq!(log, format!("{};Log message", datetime_now).green());
}

#[test]
fn test_log_with_info_message() {
    let logger = ConsoleLogger::new("%i;%m");
    
    let log = logger.log("Log message");

    assert_eq!(log, "Log;Log message".green());
}

#[test]
fn test_log_just_message() {
    let logger = ConsoleLogger::new("%m - %m");
    
    let log = logger.log("Log message");

    assert_eq!(log, "Log message - Log message".green());
}

/* 
Debug messages tests
*/
#[test]
fn test_debug_with_date() {
    let logger = ConsoleLogger::new("%dt");
    
    let debug = logger.debug("Debug Message");
    let datetime_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    assert_eq!(debug, format!("{}", datetime_now).purple());
}

#[test]
fn test_debug_with_info_message() {
    let logger = ConsoleLogger::new("%i -> %m");
    
    let debug = logger.debug("Debug message");

    assert_eq!(debug, "Debug -> Debug message".purple());
}

#[test]
fn test_debug_message_datetime_info() {
    let logger = ConsoleLogger::new("%m - %m -> %dt (%i)");
    
    let debug = logger.debug("Debug message");
    let datetime_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    assert_eq!(debug, format!("Debug message - Debug message -> {} (Debug)", datetime_now).purple());
}

/* 
Error messages tests
*/
#[test]
fn test_error_with_datetime() {
    let logger = ConsoleLogger::new("%m %dt");
    
    let error = logger.error("An error occurred at");
    let datetime_now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    assert_eq!(error, format!("An error occurred at {}", datetime_now).red());
}

#[test]
fn test_error_just_message() {
    let logger = ConsoleLogger::new("%m");
    
    let error = logger.error("Error message");

    assert_eq!(error, "Error message".red());
}

#[test]
fn test_error_message_date_info() {
    let logger = ConsoleLogger::new("%m;%d;%i");
    
    let debug = logger.debug("Error message");
    let date_now = Local::now().format("%Y-%m-%d").to_string();

    assert_eq!(debug, format!("Error message;{};Error", date_now).purple());
}