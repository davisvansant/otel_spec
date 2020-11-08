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
        // unimplemented!()
        self.trace_state.get(&key)
    }

    pub fn add(&mut self, key: String, value: String) -> Option<String> {
        // unimplemented!()
        self.trace_state.insert(key, value)
    }

    pub fn update(&mut self, key: String, value: String) -> Option<String> {
        // unimplemented!()
        self.trace_state.insert(key, value)
    }

    pub fn delete(&mut self, key: &str) -> Option<(String, String)> {
        // unimplemented!()
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
}
