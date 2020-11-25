pub use crate::metrics::instruments::Instrument;

pub struct Meter {
    pub name: String,
    pub metric_instrument: Instrument,
}

impl Meter {
    pub fn new(name: String, metric_instrument: Instrument) -> Meter {
        Meter {
            name,
            metric_instrument,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let counter = Meter::new(String::from("test_counter"), Instrument::Counter);
        assert_eq!(counter.metric_instrument, Instrument::Counter);
        let up_down_counter = Meter::new(
            String::from("test_up_down_counter"),
            Instrument::UpDownCounter,
        );
        assert_eq!(up_down_counter.metric_instrument, Instrument::UpDownCounter);
        let value_recorder = Meter::new(
            String::from("test_value_recorder"),
            Instrument::ValueRecorder,
        );
        assert_eq!(value_recorder.metric_instrument, Instrument::ValueRecorder);
        let sum_observer = Meter::new(String::from("test_sum_observer"), Instrument::SumObserver);
        assert_eq!(sum_observer.metric_instrument, Instrument::SumObserver);
        let up_down_sum_observer = Meter::new(
            String::from("test_up_down_counter"),
            Instrument::UpDownSumObserver,
        );
        assert_eq!(
            up_down_sum_observer.metric_instrument,
            Instrument::UpDownSumObserver,
        );
        let value_observer = Meter::new(
            String::from("test_value_observer"),
            Instrument::ValueObserver,
        );
        assert_eq!(value_observer.metric_instrument, Instrument::ValueObserver);
    }
}
