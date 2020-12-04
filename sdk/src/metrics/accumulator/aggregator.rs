// use api::metrics::Event;
use api::metrics::meter::Meter;

pub struct Aggregator {
    pub instance: Vec<Meter>,
}

impl Aggregator {
    pub fn init() -> Aggregator {
        Aggregator {
            instance: Vec::with_capacity(10),
        }
    }

    pub fn update(&mut self) {
        unimplemented!()
    }

    pub fn synchronized_move(&mut self) -> Vec<Meter> {
        let snapshot = self.instance.to_vec();
        self.instance.clear();
        snapshot
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::metrics::meter::Instrument;

    #[test]
    fn init() {
        let test_aggregator = Aggregator::init();
        assert_eq!(test_aggregator.instance.len(), 0);
    }

    #[test]
    fn synchronized_move() {
        let mut test_aggregator = Aggregator::init();
        assert_eq!(test_aggregator.instance.len(), 0);
        let test_meter = Meter::new(String::from("test_meter"), Instrument::Counter);
        test_aggregator.instance.push(test_meter);
        assert_eq!(test_aggregator.instance.len(), 1);
        let test_snapshot = test_aggregator.synchronized_move();
        assert_eq!(test_aggregator.instance.len(), 0);
        assert_eq!(test_snapshot.len(), 1);
    }
}
