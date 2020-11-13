pub mod counter;
pub mod up_down_counter;
pub mod value_recorder;

#[derive(Debug, PartialEq)]
pub enum Instrument {
    Counter,
    UpDownCounter,
    ValueRecorder,
    SumObserver,
    UpDownSumObserver,
    ValueObserver,
}

#[derive(PartialEq)]
pub enum Increment {
    Up,
    Down,
}
