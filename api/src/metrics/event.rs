use crate::HashMap;
use crate::SystemTime;

pub struct Event {
    pub timestamp: SystemTime,
    pub instrument_definition: String,
    pub label_set: HashMap<String, String>,
    pub value: i8,
    pub resources: String,
}

impl Event {
    pub fn default(
        instrument_definition: String,
        // label_set: String,
        value: i8,
        resources: String,
    ) -> Event {
        Event {
            timestamp: SystemTime::now(),
            instrument_definition,
            // label_set,
            label_set: HashMap::with_capacity(10),
            value,
            resources,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let test_instrument_definition = String::from("test_instrument_definition");
        // let test_label_set = String::from("test_label_set");
        let test_value = 1;
        let test_resources = String::from("test_resources");
        let test_event = Event::default(
            test_instrument_definition,
            // test_label_set,
            test_value,
            test_resources,
        );

        assert_ne!(test_event.timestamp, SystemTime::now());
        assert_eq!(
            test_event.instrument_definition,
            String::from("test_instrument_definition")
        );
        // assert_eq!(test_event.label_set, String::from("test_label_set"));
        assert_eq!(test_event.label_set.is_empty(), true);
        assert_eq!(test_event.value, 1);
        assert_eq!(test_event.resources, String::from("test_resources"));
    }
}
