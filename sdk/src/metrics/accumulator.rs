use crate::metrics::processor::Processor;

pub struct Aggregator {
    pub measurement: String,
}

impl Aggregator {
    pub fn init() -> Aggregator {
        Aggregator {
            measurement: String::with_capacity(32),
        }
    }

    pub fn update(&mut self) {
        unimplemented!()
    }

    pub fn synchronized_move(&mut self) {
        unimplemented!()
    }
}

pub struct AggregatorSnapshot {
    pub aggregator: Vec<String>,
}

impl AggregatorSnapshot {
    pub fn init() -> AggregatorSnapshot {
        AggregatorSnapshot {
            aggregator: Vec::with_capacity(10),
        }
    }
}

pub struct AggregatorSelector {
    pub aggregator: Aggregator,
}

impl AggregatorSelector {
    pub fn init() -> AggregatorSelector {
        AggregatorSelector {
            aggregator: Aggregator::init(),
        }
    }
}

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

pub struct Aggregation {
    pub result: Aggregator,
}

pub enum AggregationKind {
    SomeAggregationKind,
}

pub struct Accumulator {
    pub aggregator: Vec<Accumulation>,
    // pub accumulation: (String, String),
    // accumulation: Accumulation,
}

impl Accumulator {
    pub fn init() -> Accumulator {
        Accumulator {
            aggregator: Vec::with_capacity(10),
            // accumulation: (String::with_capacity(32), String::with_capacity(32)),
            // accumulation: Accumulation {
            //     instrument: String::with_capacity(32),
            //     label_set: String::with_capacity(32),
            //     resource: String::with_capacity(32),
            //     aggregator_snapshot: AggregatorSnapshot::init(),
            // },
        }
    }

    pub fn collect(&mut self, accumulation: Accumulation, _processor: Processor) {
        self.aggregator.push(accumulation);
        // _processor.process()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregator_init() {
        let test_aggregator = Aggregator::init();
        assert_eq!(test_aggregator.measurement, String::from(""));
    }

    #[test]
    fn aggregator_snapshot_init() {
        let test_aggregator_snapshot = AggregatorSnapshot::init();
        assert_eq!(test_aggregator_snapshot.aggregator.len(), 0);
    }

    #[test]
    fn aggregator_selector_init() {
        let test_aggregator_selector = AggregatorSelector::init();
        assert_eq!(
            test_aggregator_selector.aggregator.measurement,
            String::from("")
        );
    }

    #[test]
    fn accumlation_init() {
        let test_accumulation = Accumulation::init();
        assert_eq!(test_accumulation.instrument, String::from(""));
        assert_eq!(test_accumulation.label_set, String::from(""));
        assert_eq!(test_accumulation.resource, String::from(""));
        assert_eq!(test_accumulation.aggregator_snapshot.aggregator.len(), 0);
    }

    #[test]
    fn accumlator_init() {
        let test_accumulator = Accumulator::init();
        assert_eq!(test_accumulator.aggregator.len(), 0);
    }
}
