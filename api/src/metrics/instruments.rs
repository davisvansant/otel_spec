pub mod counter;

#[derive(Debug, PartialEq)]
pub enum Instrument {
    Counter,
    UpDownCounter,
    ValueRecorder,
    SumObserver,
    UpDownSumObserver,
    ValueObserver,
}
