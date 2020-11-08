use crate::HashMap;

pub struct TraceState {
    pub trace_state: HashMap<String, String>,
}

impl TraceState {
    pub fn create() -> Self {
        Self {
            trace_state: HashMap::with_capacity(10),
        }
    }

    pub fn get(&self, key: String) -> Option<&String> {
        self.trace_state.get(&key)
    }

    pub fn add(&mut self, key: String, value: String) -> Option<String> {
        self.trace_state.insert(key, value)
    }

    pub fn update(&mut self, key: String, value: String) -> Option<String> {
        self.trace_state.insert(key, value)
    }

    pub fn delete(&mut self, key: &str) -> Option<(String, String)> {
        self.trace_state.remove_entry(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let trace_state = TraceState::create();
        assert!(trace_state.trace_state.is_empty());
    }

    #[test]
    fn get() {
        let mut trace_state = TraceState::create();
        trace_state
            .trace_state
            .insert(String::from("test_key"), String::from("test_value"));
        let test_get = trace_state.get(String::from("test_key"));
        assert_eq!(test_get.unwrap(), &String::from("test_value"));
    }

    #[test]
    fn add() {
        let mut trace_state = TraceState::create();
        assert_eq!(trace_state.trace_state.len(), 0);
        let test_key = String::from("test_key");
        let test_value = String::from("test_value");
        trace_state.add(test_key, test_value);
        assert_eq!(trace_state.trace_state.len(), 1);
    }

    #[test]
    fn update() {
        let mut trace_state = TraceState::create();
        let test_key = String::from("test_key");
        let test_value = String::from("test_value");
        trace_state.add(test_key, test_value);
        let tested_value = trace_state.get(String::from("test_key")).unwrap();
        assert_eq!(tested_value, &String::from("test_value"));
        let updated_key = String::from("test_key");
        let updated_value = String::from("test_updated_value");
        trace_state.update(updated_key, updated_value);
        let test_updated_value = trace_state.get(String::from("test_key")).unwrap();
        assert_eq!(test_updated_value, &String::from("test_updated_value"));
    }

    #[test]
    fn delete() {
        let mut trace_state = TraceState::create();
        let test_key = String::from("test_key");
        let test_value = String::from("test_value");
        trace_state.add(test_key, test_value);
        let test_get = trace_state.get(String::from("test_key"));
        assert_eq!(test_get.unwrap(), &String::from("test_value"));
        let delete_key = String::from("test_key");
        trace_state.delete(&delete_key);
        let test_get_again = trace_state.get(String::from("test_key"));
        assert_eq!(test_get_again, None);
    }
}
