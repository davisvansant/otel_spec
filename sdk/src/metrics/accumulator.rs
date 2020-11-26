pub struct Aggregator {
    pub measurement: String,
}

pub struct AggregatorSnapshot {
    pub aggregator: Vec<String>,
}

pub struct AggregatorSelector {
    pub aggregator: Aggregator,
}

pub struct Accumluation {
    pub instrument: String,
    pub label_set: String,
    pub resource: String,
    pub aggregator_snapshot: AggregatorSnapshot,
}

pub struct Aggregation {
    pub result: Aggregator,
}

pub enum AggregationKind {
    SomeAggregationKind,
}

pub struct Accumulator {
    pub aggregator: Vec<String>,
    pub accumulation: (String, String),
}

impl Accumulator {
    pub fn init() -> Accumulator {
        Accumulator {
            aggregator: Vec::with_capacity(10),
            accumulation: (String::with_capacity(32), String::with_capacity(32)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let test_accumulator = Accumulator::init();
        assert_eq!(test_accumulator.aggregator.len(), 0);
        assert_eq!(
            test_accumulator.accumulation,
            (String::from(""), String::from(""))
        );
    }
}
