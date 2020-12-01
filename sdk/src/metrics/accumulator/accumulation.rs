use super::AggregatorSnapshot;

pub struct Accumulation {
    pub instrument: String,
    pub label_set: String,
    pub resource: String,
    pub aggregator_snapshot: AggregatorSnapshot,
}

impl Accumulation {
    pub fn init() -> Accumulation {
        Accumulation {
            instrument: String::with_capacity(32),
            label_set: String::with_capacity(32),
            resource: String::with_capacity(32),
            aggregator_snapshot: AggregatorSnapshot::init(),
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
        assert_eq!(test_accumulation.label_set, String::from(""));
        assert_eq!(test_accumulation.resource, String::from(""));
        assert_eq!(test_accumulation.aggregator_snapshot.aggregator.len(), 0);
    }
}
