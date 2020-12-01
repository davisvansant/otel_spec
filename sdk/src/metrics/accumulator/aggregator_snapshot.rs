use api::metrics::Event;

pub struct AggregatorSnapshot {
    pub aggregator: Vec<Event>,
}

impl AggregatorSnapshot {
    pub fn init() -> AggregatorSnapshot {
        AggregatorSnapshot {
            aggregator: Vec::with_capacity(10),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let test_aggregator_snapshot = AggregatorSnapshot::init();
        assert_eq!(test_aggregator_snapshot.aggregator.len(), 0);
    }
}
