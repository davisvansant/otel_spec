#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum StatusCode {
    Unset,
    Ok,
    Error,
}

#[derive(Clone, PartialEq)]
pub struct Status {
    pub status_code: StatusCode,
    pub description: Option<String>,
}

impl Status {
    pub fn default() -> Status {
        Status {
            status_code: StatusCode::Unset,
            description: None,
        }
    }

    pub fn set(&mut self, status_code: StatusCode, description: Option<String>) {
        self.status_code = status_code;
        self.description = description;
    }
}
