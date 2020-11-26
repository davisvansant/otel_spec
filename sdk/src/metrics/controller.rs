use crate::metrics::accumulator::Accumulator;
use crate::metrics::exporter::Exporter;
use crate::metrics::processor::Processor;

pub struct Controller {
    pub accumulator: Accumulator,
    pub processor: Processor,
    pub exporter: Exporter,
}

impl Controller {
    pub fn init() -> Controller {
        Controller {
            accumulator: Accumulator::init(),
            processor: Processor::init(),
            exporter: Exporter::init(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let test_controller = Controller::init();
        assert_eq!(test_controller.accumulator.aggregator.len(), 0);
        assert_eq!(test_controller.processor.some_collection.len(), 0);
        assert_eq!(test_controller.exporter.some_collection.len(), 0);
    }
}
