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

    #[test]
    fn init() {
        let test_aggregator = Aggregator::init();
        assert_eq!(test_aggregator.instance.len(), 0);
    }
}
