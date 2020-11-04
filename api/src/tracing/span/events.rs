use crate::HashMap;
use crate::SystemTime;

pub(super) struct Event {
    pub(super) name: String,
    pub(super) timestamp: SystemTime,
    pub(super) attributes: Vec<HashMap<String, String>>,
}

impl Event {
    pub(super) fn new(name: String) -> Event {
        Event {
            name,
            timestamp: SystemTime::now(),
            attributes: Vec::new(),
        }
    }
}
