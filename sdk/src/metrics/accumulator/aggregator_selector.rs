// use super::Aggregator;
use super::AggregationKind;

pub struct AggregatorSelector {
    pub aggregator: AggregationKind,
}

impl AggregatorSelector {
    pub fn aggregator_for(aggregator: AggregationKind) -> AggregatorSelector {
        AggregatorSelector { aggregator }
    }
}

// pub struct AggregatorSelector {
//     pub aggregator: Aggregator,
// }

// impl AggregatorSelector {
//     pub fn init() -> AggregatorSelector {
//         AggregatorSelector {
//             aggregator: Aggregator::init(),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn init() {
    //     let test_aggregator_selector = AggregatorSelector::init();
    //     assert_eq!(test_aggregator_selector.aggregator.instance.len(), 0);
    // }
    #[test]
    fn aggregator_for_sum() {
        let test_aggregator_for_sum = AggregatorSelector::aggregator_for(AggregationKind::Sum);
        assert_eq!(test_aggregator_for_sum.aggregator, AggregationKind::Sum);
    }

    #[test]
    fn aggregator_for_last_value() {
        let test_aggregator_for_last_value =
            AggregatorSelector::aggregator_for(AggregationKind::LastValue);
        assert_eq!(
            test_aggregator_for_last_value.aggregator,
            AggregationKind::LastValue,
        );
    }

    #[test]
    fn aggregator_for_histogram() {
        let test_aggregator_for_histogram =
            AggregatorSelector::aggregator_for(AggregationKind::Histogram);
        assert_eq!(
            test_aggregator_for_histogram.aggregator,
            AggregationKind::Histogram,
        );
    }

    #[test]
    fn aggregator_for_min_max_sum_count() {
        let test_aggregator_for_min_max_sum_count =
            AggregatorSelector::aggregator_for(AggregationKind::MinMaxSumCount);
        assert_eq!(
            test_aggregator_for_min_max_sum_count.aggregator,
            AggregationKind::MinMaxSumCount,
        );
    }

    #[test]
    fn aggregator_for_exact() {
        let test_aggregator_for_exact = AggregatorSelector::aggregator_for(AggregationKind::Exact);
        assert_eq!(test_aggregator_for_exact.aggregator, AggregationKind::Exact);
    }

    #[test]
    fn aggregator_for_sketch() {
        let test_aggregator_for_sketch =
            AggregatorSelector::aggregator_for(AggregationKind::Sketch);
        assert_eq!(
            test_aggregator_for_sketch.aggregator,
            AggregationKind::Sketch,
        );
    }
}
