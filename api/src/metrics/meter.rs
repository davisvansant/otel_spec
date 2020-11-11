use crate::metrics::instruments::Instrument;

pub struct Meter {
    pub metric_instrument: Instrument,
}

impl Meter {
    pub fn new(instrument: Instrument) -> Meter {
        Meter {
            metric_instrument: instrument,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let counter = Meter::new(Instrument::Counter);
        assert_eq!(counter.metric_instrument, Instrument::Counter);
        let up_down_counter = Meter::new(Instrument::UpDownCounter);
        assert_eq!(up_down_counter.metric_instrument, Instrument::UpDownCounter);
        let value_recorder = Meter::new(Instrument::ValueRecorder);
        assert_eq!(value_recorder.metric_instrument, Instrument::ValueRecorder);
        let sum_observer = Meter::new(Instrument::SumObserver);
        assert_eq!(sum_observer.metric_instrument, Instrument::SumObserver);
        let up_down_sum_observer = Meter::new(Instrument::UpDownSumObserver);
        assert_eq!(
            up_down_sum_observer.metric_instrument,
            Instrument::UpDownSumObserver
        );
        let value_observer = Meter::new(Instrument::ValueObserver);
        assert_eq!(value_observer.metric_instrument, Instrument::ValueObserver);
    }
}
