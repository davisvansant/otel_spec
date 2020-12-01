use super::Aggregator;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let test_aggregator_selector = AggregatorSelector::init();
        assert_eq!(test_aggregator_selector.aggregator.instance.len(), 0);
    }
}
