// use crate::metrics::processor::Processor;
// use api::metrics::Event;

pub use self::accumulation::Accumulation;
pub use self::aggregation::Aggregation;
pub use self::aggregator::Aggregator;
pub use self::aggregator_selector::AggregatorSelector;
pub use self::aggregator_snapshot::AggregatorSnapshot;

mod accumulation;
mod aggregation;
mod aggregator;
mod aggregator_selector;
mod aggregator_snapshot;

// pub struct Aggregator {
//     pub instance: Vec<Event>,
// }
// impl Aggregator {
//     pub fn init() -> Aggregator {
//         Aggregator {
//             instance: Vec::with_capacity(10),
//         }
//     }
//
//     pub fn update(&mut self) {
//         unimplemented!()
//     }
//
//     pub fn synchronized_move(&mut self) -> Vec<Event> {
//         let snapshot = self.instance.to_vec();
//         self.instance.clear();
//         snapshot
//     }
// }

// pub struct AggregatorSnapshot {
//     pub aggregator: Vec<Event>,
// }
//
// impl AggregatorSnapshot {
//     pub fn init() -> AggregatorSnapshot {
//         AggregatorSnapshot {
//             aggregator: Vec::with_capacity(10),
//         }
//     }
// }

// pub struct AggregatorSelector {
//     pub aggregator: Aggregator,
// }
//
// impl AggregatorSelector {
//     pub fn init() -> AggregatorSelector {
//         AggregatorSelector {
//             aggregator: Aggregator::init(),
//         }
//     }
// }

// pub struct Accumulation {
//     pub instrument: String,
//     pub label_set: String,
//     pub resource: String,
//     pub aggregator_snapshot: AggregatorSnapshot,
// }
//
// impl Accumulation {
//     pub fn init() -> Accumulation {
//         Accumulation {
//             instrument: String::with_capacity(32),
//             label_set: String::with_capacity(32),
//             resource: String::with_capacity(32),
//             aggregator_snapshot: AggregatorSnapshot::init(),
//         }
//     }
// }

// pub struct Aggregation {
//     pub result: Aggregator,
// }
//
// pub enum AggregationKind {
//     SomeAggregationKind,
// }

pub struct Accumulator {
    pub aggregator: Aggregator,
    pub snapshot: AggregatorSnapshot,
}

impl Accumulator {
    pub fn init() -> Accumulator {
        Accumulator {
            aggregator: Aggregator::init(),
            snapshot: AggregatorSnapshot::init(),
        }
    }

    // pub fn collect(&mut self, _accumulation: Accumulation, _processor: Processor) {
    pub fn collect(&mut self) {
        // self.aggregator.push(accumulation);
        // self.aggregator.instance.push(accumulation);
        // _processor.process
        self.snapshot.aggregator = self.aggregator.synchronized_move()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::metrics::Event;

    // #[test]
    // fn aggregator_init() {
    //     let test_aggregator = Aggregator::init();
    //     // assert_eq!(test_aggregator.measurement, String::from(""));
    //     assert_eq!(test_aggregator.instance.len(), 0);
    // }
    //
    // #[test]
    // fn aggregator_snapshot_init() {
    //     let test_aggregator_snapshot = AggregatorSnapshot::init();
    //     assert_eq!(test_aggregator_snapshot.aggregator.len(), 0);
    // }
    //
    // #[test]
    // fn aggregator_selector_init() {
    //     let test_aggregator_selector = AggregatorSelector::init();
    //     assert_eq!(test_aggregator_selector.aggregator.instance.len(), 0);
    //     // test_aggregator_selector.aggregator.measurement,
    // }

    // #[test]
    // fn accumlation_init() {
    //     let test_accumulation = Accumulation::init();
    //     assert_eq!(test_accumulation.instrument, String::from(""));
    //     assert_eq!(test_accumulation.label_set, String::from(""));
    //     assert_eq!(test_accumulation.resource, String::from(""));
    //     assert_eq!(test_accumulation.aggregator_snapshot.aggregator.len(), 0);
    // }

    #[test]
    fn accumlator_init() {
        let test_accumulator = Accumulator::init();
        // assert_eq!(test_accumulator.aggregator.len(), 0);
        // assert_eq!(test_accumulator.map.len(), 0);
        assert_eq!(test_accumulator.aggregator.instance.len(), 0);
    }

    #[test]
    fn accumlator_collect() {
        let mut test_accumulator = Accumulator::init();
        assert_eq!(test_accumulator.aggregator.instance.len(), 0);
        let test_event_one = Event::default(
            String::from("test_instrument_definition"),
            1,
            String::from("test_resources"),
        );
        test_accumulator.aggregator.instance.push(test_event_one);
        assert_eq!(test_accumulator.aggregator.instance.len(), 1);
        assert_eq!(test_accumulator.snapshot.aggregator.len(), 0);
        test_accumulator.collect();
        assert_eq!(test_accumulator.aggregator.instance.len(), 0);
        assert_eq!(test_accumulator.snapshot.aggregator.len(), 1);
    }
}
