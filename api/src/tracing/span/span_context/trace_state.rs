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

    pub fn get() {
        unimplemented!()
    }

    pub fn add() {
        unimplemented!()
    }

    pub fn update() {
        unimplemented!()
    }

    pub fn delete() {
        unimplemented!()
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
