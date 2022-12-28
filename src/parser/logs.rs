#[derive(Eq, PartialEq)]
enum ErrorType {
    UNKNOWN,
    MINOR,
    MAJOR
}

#[derive(Eq, PartialEq)]
pub struct Log {
    file_name: String,
    file_path: Vec<String>,
    error_type: ErrorType,
    error_code: String,
    error_line: usize
}

impl From<(&str, String)> for Log {
    fn from(log: (&str, String)) -> Self
    {
        let data: Vec<String> = log.1.split(':').map(|elem| {
            elem.replace(" ", "").to_string()
        }).collect();

        if data.len() == 4 {
            Log {
                file_name: if let Some(name) = data[0].split('/').last() {
                    name.to_string()
                } else {
                    data[0].to_string()
                },
                file_path: data[0].split('/').map(|elem| elem.to_string()).collect(),
                error_type: match data[2].as_str() {
                    "MINOR" => ErrorType::MINOR,
                    "MAJOR" => ErrorType::MAJOR,
                    _ => ErrorType::UNKNOWN
                },
                error_code: data[3].to_string(),
                error_line: data[1].parse::<usize>().unwrap_or(0)
            }
        } else {
            Log {
                file_name: "unknown".to_string(),
                file_path: vec![],
                error_type: ErrorType::UNKNOWN,
                error_code: "unknown".to_string(),
                error_line: 0,
            }
        }
    }
}

impl Log {
    fn is_correctly_parsed(&self) -> bool
    {
        *self != Log {
            file_name: "unknown".to_string(),
            file_path: vec![],
            error_type: ErrorType::UNKNOWN,
            error_code: "unknown".to_string(),
            error_line: 0,
        }
    }
}

pub fn build_log(current_working_directory: &str, buffer: String) -> Vec<Log>
{
    let mut logs: Vec<Log> = vec![];

    for line in buffer.split('\n').map(|line| line.to_string()) {

        let log: Log = Log::from(
            (current_working_directory, line)
        );

        if log.is_correctly_parsed() {
            logs.push(log);
        }
    }

    logs
}
