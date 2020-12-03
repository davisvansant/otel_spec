pub use self::accumulation::Accumulation;
pub use self::aggregation::Aggregation;
pub use self::aggregation::AggregationKind;
pub use self::aggregator::Aggregator;
pub use self::aggregator_selector::AggregatorSelector;
pub use self::aggregator_snapshot::AggregatorSnapshot;

mod accumulation;
mod aggregation;
mod aggregator;
mod aggregator_selector;
mod aggregator_snapshot;

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
        // let record = (Event.label_set, )
        self.snapshot.aggregator = self.aggregator.synchronized_move();
        // let accumulation = Accumulation::init();
        // for event in self.snapshot.aggregator.iter_mut() {
        //     let mut accumulation = Accumulation::init();
        //     event.instrument_definition = accumulation.instrument;
        //     event.label_set = accumulation.label_set;
        //     event.resources = accumulation.resource;
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use api::metrics::Event;
    use api::metrics::meter::{Instrument, Meter};

    #[test]
    fn init() {
        let test_accumulator = Accumulator::init();
        assert_eq!(test_accumulator.aggregator.instance.len(), 0);
    }

    #[test]
    fn collect() {
        let mut test_accumulator = Accumulator::init();
        assert_eq!(test_accumulator.aggregator.instance.len(), 0);
        // let test_event_one = Event::default(
        //     String::from("test_instrument_definition"),
        //     1,
        //     String::from("test_resources"),
        // );
        // test_accumulator.aggregator.instance.push(test_event_one);
        let test_meter = Meter::new(String::from("test_counter"), Instrument::Counter);
        test_accumulator.aggregator.instance.push(test_meter);
        assert_eq!(test_accumulator.aggregator.instance.len(), 1);
        assert_eq!(test_accumulator.snapshot.aggregator.len(), 0);
        test_accumulator.collect();
        assert_eq!(test_accumulator.aggregator.instance.len(), 0);
        assert_eq!(test_accumulator.snapshot.aggregator.len(), 1);
    }
}
