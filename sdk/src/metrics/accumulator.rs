pub struct Accumulator {
    pub aggregator: Vec<String>,
}

impl Accumulator {
    pub fn init() -> Accumulator {
        Accumulator {
            aggregator: Vec::with_capacity(10),
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
    }
}
