use super::AggregatorSnapshot;
use crate::HashMap;
use api::metrics::Event;

pub struct Accumulation {
    pub instrument: String,
    // pub label_set: String,
    pub label_set: HashMap<String, String>,
    pub resource: String,
    // pub aggregator_snapshot: AggregatorSnapshot,
    pub aggregator_snapshot: Vec<Event>,
}

impl Accumulation {
    pub fn init() -> Accumulation {
        Accumulation {
            instrument: String::with_capacity(32),
            // label_set: String::with_capacity(32),
            label_set: HashMap::with_capacity(10),
            resource: String::with_capacity(32),
            // aggregator_snapshot: AggregatorSnapshot::init(),
            aggregator_snapshot: Vec::with_capacity(10),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let test_accumulation = Accumulation::init();
        assert_eq!(test_accumulation.instrument, String::from(""));
        // assert_eq!(test_accumulation.label_set, String::from(""));
        assert_eq!(test_accumulation.label_set.len(), 0);
        assert_eq!(test_accumulation.resource, String::from(""));
        assert_eq!(test_accumulation.aggregator_snapshot.len(), 0);
    }
}
