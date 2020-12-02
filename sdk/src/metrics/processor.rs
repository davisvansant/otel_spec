use crate::metrics::accumulator::Accumulation;

pub struct Processor {
    pub accumulations: Vec<Accumulation>,
}

impl Processor {
    pub fn init() -> Processor {
        Processor {
            accumulations: Vec::with_capacity(10),
        }
    }

    pub fn process() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let test_processor = Processor::init();
        assert_eq!(test_processor.accumulations.len(), 0);
    }
}
