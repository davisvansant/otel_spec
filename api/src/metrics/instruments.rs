pub mod counter;
pub mod sum_observer;
pub mod up_down_counter;
pub mod up_down_sum_observer;
pub mod value_observer;
pub mod value_recorder;

#[derive(Clone, Debug, PartialEq)]
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
