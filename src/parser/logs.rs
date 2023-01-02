use std::{fs, fs::Metadata};

#[derive(Eq, PartialEq, Debug)]
pub enum ErrorType {
    UNKNOWN,
    MINOR,
    MAJOR
}

#[derive(Debug)]
pub struct Log {
    pub file_name: String,
    pub file_path: Vec<String>,
    pub error_type: ErrorType,
    pub error_code: String,
    pub error_line: usize,
    pub metadata: Option<Metadata>
}

impl From<String> for Log {

    fn from(log: String) -> Self
    {
        let data: Vec<String> = log.split(':').map(|elem| {
            elem.replace(" ", "").to_string()
        }).collect();

        let unknown: Log = Log {
            file_name: "unknown".to_string(),
            file_path: vec![],
            error_type: ErrorType::UNKNOWN,
            error_code: "unknown".to_string(),
            error_line: 0,
            metadata: None
        };

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
                error_line: data[1].parse::<usize>().unwrap_or(0),
                metadata: match fs::metadata(&data[0]) {
                    Ok(data) => Some(data),
                    Err(_) => return unknown
                }
            }
        } else {
            unknown
        }
    }
}

impl Log {

    fn is_correctly_parsed(&self) -> bool
    {
        let unknown: Log = Log {
            file_name: "unknown".to_string(),
            file_path: vec![],
            error_type: ErrorType::UNKNOWN,
            error_code: "unknown".to_string(),
            error_line: 0,
            metadata: None
        };

        self.file_name != unknown.file_name
            || self.file_path != unknown.file_path
            || self.error_type != unknown.error_type
            || self.error_code != unknown.error_code
            || self.error_line != unknown.error_line
    }

    pub fn build_log(buffer: String) -> Vec<Log>
    {
        let mut logs: Vec<Log> = vec![];

        for line in buffer.split('\n').map(|line| line.to_string()) {

            let log: Log = Log::from(line);

            if log.is_correctly_parsed() {
                logs.push(log);
            }
        }

        logs
    }
}
