#[derive(Debug)]
enum ErrorType {
    MINOR,
    MAJOR,
    UNKNOWN
}

#[derive(Debug)]
pub struct Log {
    path: String,
    row: u64,
    err_type: ErrorType,
    code: String,
    description: String
}

impl From<Vec<String>> for Log {
    fn from(value: Vec<String>) -> Self {
        Log {
            path: value[0].clone(),
            row: value[1].parse::<u64>().unwrap_or(0),
            err_type: match value[2].as_str() {
                "MINOR" => ErrorType::MINOR,
                "MAJOR" => ErrorType::MAJOR,
                _ => ErrorType::UNKNOWN
            },
            code: value[3].clone(),
            description: "In development, coming soon (;".to_string(),
        }
    }
}

impl Log {

    pub fn parse(buffer: String) -> Result<Vec<Log>, String> {
        let mut result: Vec<Log> = vec![];

        for line in buffer.split('\n') {
            if line.len() == 0 {
                continue
            }
            let mut log: Vec<String> = vec![];
            for data in line.split(':') {
                log.push(data.to_string());
            }
            if log.len() != 4 {
                return Err("Le format des logs n est pas supporte !".to_string())
            }
            log[2] = log[2].replace(" ", "");
            result.push(log.into());
        }

        Ok(result)
    }

    pub fn get_path(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for elem in self.path.split('/') {
            if elem != "." {
                result.push(elem.to_string());
            }
        }
        result
    }
}
