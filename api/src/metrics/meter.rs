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
