use std::io::stdout;
use crate::Log;

use serde_json::Value;
use crossterm::{
    execute,
    style::{Print, SetForegroundColor, ResetColor, Color, SetAttribute, Attribute}
};

use crate::parser::logs::ErrorType;

pub fn print(log: Log)
{
    match print_path(&log) { Ok(_) | Err(_) => () };
    match print_line(&log) { Ok(_) | Err(_) => () };
    match print_error_type(&log) { Ok(_) | Err(_) => () };
    match print_error_code(&log) { Ok(_) | Err(_) => () };
    match print_description(&log) { Ok(_) | Err(_) => () };
    print!("\n");
}

fn print_path(log: &Log) -> crossterm::Result<()>
{
    let path: String = log.file_path.join("/");

    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetAttribute(Attribute::Bold),
        Print(format!("{path}\n")),
        ResetColor,
        SetAttribute(Attribute::Reset)
    )
}

fn print_line(log: &Log) -> crossterm::Result<()>
{
    execute!(
        stdout(),
        Print("├── At line ".to_string()),
        SetAttribute(Attribute::Bold),
        Print(format!("{}\n", log.error_line)),
        SetAttribute(Attribute::Reset)
    )
}

fn print_error_type(log: &Log) -> crossterm::Result<()>
{
    match log.error_type {
        ErrorType::UNKNOWN => execute!(
            stdout(),
            Print("├── Unknown error severity".to_string())
        ),
        ErrorType::MINOR => execute!(
            stdout(),
            Print("├── Error severity is ".to_string()),
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Yellow),
            Print("MINOR\n".to_string()),
            SetAttribute(Attribute::Reset),
            ResetColor
        ),
        ErrorType::MAJOR => execute!(
            stdout(),
            Print("├── Error severity is ".to_string()),
            SetAttribute(Attribute::Bold),
            SetForegroundColor(Color::Red),
            Print("MAJOR\n".to_string()),
            SetAttribute(Attribute::Reset),
            ResetColor
        )
    }
}

pub fn print_error_code(log: &Log) -> crossterm::Result<()>
{
    execute!(
        stdout(),
        Print("├── Error reference is ".to_string()),
        SetAttribute(Attribute::Bold),
        Print(format!("{}\n", log.error_code)),
        SetAttribute(Attribute::Reset)
    )
}

pub fn print_description(log: &Log) -> crossterm::Result<()>
{
    let description_json_str: &str = include_str!("../data/coding_style_errors_FR.json");

    let json_data: Value = match serde_json::from_str(description_json_str) {
        Ok(value) => value,
        Err(_) => return execute!(
            stdout(),
            Print("└── No description was found".to_string())
        )
    };

    execute!(
        stdout(),
        Print("└── ".to_string()),
        SetAttribute(Attribute::Bold),
        Print(format!("{}\n", json_data[&log.error_code])),
        SetAttribute(Attribute::Reset)
    )
}
