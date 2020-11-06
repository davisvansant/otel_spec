use crate::HashMap;
use crate::SystemTime;

pub struct Event {
    pub name: String,
    pub timestamp: SystemTime,
    pub attributes: Vec<HashMap<String, String>>,
}

impl Event {
    pub fn new(name: String) -> Event {
        Event {
            name,
            timestamp: SystemTime::now(),
            attributes: Vec::new(),
        }
    }
}
