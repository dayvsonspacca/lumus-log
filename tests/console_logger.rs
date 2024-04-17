use chrono::Local;
use colored::{Color, Colorize};
use lumus_logger::ConsoleLogger;

#[test]
fn test_warn_verbose_true() {
    let logger = ConsoleLogger::new(true);
    let message = "This is a warning message.";

    logger.warn(message);

    let expected_output = format!(
        "[ {} - Warning ] - {}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        message
    )
    .yellow();
    assert!(output_contains_color(expected_output.to_string(), Color::Yellow));
}

#[test]
fn test_warn_verbose_false() {
    let logger = ConsoleLogger::new(false);
    let message = "This is a warning message.";

    logger.warn(message);

    let expected_output = format!("[ Warning ] - {}\n", message).yellow();
    assert!(output_contains_color(expected_output.to_string(), Color::Yellow));
}

#[test]
fn test_info_verbose_true() {
    let logger = ConsoleLogger::new(true);
    let message = "This is an informational message.";

    logger.info(message);

    let expected_output = format!(
        "[ {} - Info ] - {}\n",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        message
    )
    .cyan();
    assert!(output_contains_color(expected_output.to_string(), Color::Cyan));
}

#[test]
fn test_info_verbose_false() {
    let logger = ConsoleLogger::new(false);
    let message = "This is an informational message.";

    logger.info(message);

    let expected_output = format!("[ Info ] - {}\n", message).cyan();
    assert!(output_contains_color(expected_output.to_string(), Color::Cyan));
}

// Helper function to check if output contains specified color
fn output_contains_color(output: String, color: Color) -> bool {
    output.contains(&format!("{}", color.to_fg_str()))
}