use super::Aggregator;

pub struct Aggregation {
    pub result: Aggregator,
}

#[derive(Debug, PartialEq)]
pub enum AggregationKind {
    // SomeAggregationKind,
    Sum,
    LastValue,
    Histogram,
    MinMaxSumCount,
    Exact,
    Sketch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregation_kind() {
        let test_sum = AggregationKind::Sum;
        let test_last_value = AggregationKind::LastValue;
        let test_histogram = AggregationKind::Histogram;
        let test_min_max_sum_count = AggregationKind::MinMaxSumCount;
        let test_exact = AggregationKind::Exact;
        let test_sketch = AggregationKind::Sketch;
        assert_eq!(test_sum, AggregationKind::Sum);
        assert_eq!(test_last_value, AggregationKind::LastValue);
        assert_eq!(test_histogram, AggregationKind::Histogram);
        assert_eq!(test_min_max_sum_count, AggregationKind::MinMaxSumCount);
        assert_eq!(test_exact, AggregationKind::Exact);
        assert_eq!(test_sketch, AggregationKind::Sketch);
    }
}
