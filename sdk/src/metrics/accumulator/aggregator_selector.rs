// use super::Aggregator;
use super::AggregationKind;
use api::metrics::meter::Instrument;
use api::metrics::meter::Meter;

pub struct AggregatorSelector {
    pub aggregator: AggregationKind,
}

impl AggregatorSelector {
    pub fn aggregator_for(aggregator: Meter) -> AggregatorSelector {
        let selected = match aggregator.metric_instrument {
            Instrument::Counter => AggregationKind::Sum,
            Instrument::UpDownCounter => AggregationKind::Sum,
            Instrument::ValueRecorder => AggregationKind::Sum,
            Instrument::SumObserver => AggregationKind::Sum,
            Instrument::UpDownSumObserver => AggregationKind::Sum,
            Instrument::ValueObserver => AggregationKind::LastValue,
        };

        AggregatorSelector {
            aggregator: selected,
        }
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
        let test_counter = Meter::new(String::from("test_counter"), Instrument::Counter);
        let test_aggregator_for_counter = AggregatorSelector::aggregator_for(test_counter);
        assert_eq!(test_aggregator_for_counter.aggregator, AggregationKind::Sum);
        let test_up_down_counter = Meter::new(
            String::from("test_up_down_counter"),
            Instrument::UpDownCounter,
        );
        let test_aggregator_for_up_down_counter =
            AggregatorSelector::aggregator_for(test_up_down_counter);
        assert_eq!(
            test_aggregator_for_up_down_counter.aggregator,
            AggregationKind::Sum,
        );
        let test_value_recorder = Meter::new(
            String::from("test_value_recorder"),
            Instrument::UpDownCounter,
        );
        let test_aggregator_for_value_recorder =
            AggregatorSelector::aggregator_for(test_value_recorder);
        assert_eq!(
            test_aggregator_for_value_recorder.aggregator,
            AggregationKind::Sum,
        );
        let test_sum_observer =
            Meter::new(String::from("test_sum_observer"), Instrument::SumObserver);
        let test_aggregator_for_sum_observer =
            AggregatorSelector::aggregator_for(test_sum_observer);
        assert_eq!(
            test_aggregator_for_sum_observer.aggregator,
            AggregationKind::Sum,
        );
        let test_up_down_sum_observer = Meter::new(
            String::from("test_up_down_sum_observer"),
            Instrument::UpDownSumObserver,
        );
        let test_aggregator_for_up_down_sum_observer =
            AggregatorSelector::aggregator_for(test_up_down_sum_observer);
        assert_eq!(
            test_aggregator_for_up_down_sum_observer.aggregator,
            AggregationKind::Sum,
        );
    }

    #[test]
    fn aggregator_for_last_value() {
        let test_value_observer = Meter::new(
            String::from("test_value_observer"),
            Instrument::ValueObserver,
        );
        let test_aggregator_for_last_value =
            AggregatorSelector::aggregator_for(test_value_observer);
        assert_eq!(
            test_aggregator_for_last_value.aggregator,
            AggregationKind::LastValue,
        );
    }

    // #[test]
    // fn aggregator_for_histogram() {
    //     let test_aggregator_for_histogram =
    //         AggregatorSelector::aggregator_for(AggregationKind::Histogram);
    //     assert_eq!(
    //         test_aggregator_for_histogram.aggregator,
    //         AggregationKind::Histogram,
    //     );
    // }
    //
    // #[test]
    // fn aggregator_for_min_max_sum_count() {
    //     let test_aggregator_for_min_max_sum_count =
    //         AggregatorSelector::aggregator_for(AggregationKind::MinMaxSumCount);
    //     assert_eq!(
    //         test_aggregator_for_min_max_sum_count.aggregator,
    //         AggregationKind::MinMaxSumCount,
    //     );
    // }
    //
    // #[test]
    // fn aggregator_for_exact() {
    //     let test_aggregator_for_exact = AggregatorSelector::aggregator_for(AggregationKind::Exact);
    //     assert_eq!(test_aggregator_for_exact.aggregator, AggregationKind::Exact);
    // }
    //
    // #[test]
    // fn aggregator_for_sketch() {
    //     let test_aggregator_for_sketch =
    //         AggregatorSelector::aggregator_for(AggregationKind::Sketch);
    //     assert_eq!(
    //         test_aggregator_for_sketch.aggregator,
    //         AggregationKind::Sketch,
    //     );
    // }
}
