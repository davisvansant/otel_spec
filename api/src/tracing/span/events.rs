use crate::HashMap;
use crate::SystemTime;

pub(crate) struct Event {
    pub(crate) name: String,
    pub(crate) timestamp: SystemTime,
    pub(crate) attributes: Vec<HashMap<String, String>>,
}

impl Event {
    pub(crate) fn new(name: String) -> Event {
        Event {
            name,
            timestamp: SystemTime::now(),
            attributes: Vec::new(),
        }
    }
}
