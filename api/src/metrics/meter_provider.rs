use crate::metrics::meter::{Instrument, Meter};

pub struct MeterProvider {
    pub name: &'static str,
    pub version: &'static str,
    pub meter: Vec<Meter>,
    pub shutdown: bool,
}

impl MeterProvider {
    pub fn default() -> MeterProvider {
        MeterProvider {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            meter: Vec::with_capacity(1),
            shutdown: false,
        }
    }

    pub fn new_meter(&mut self, name: String, metric_instrument: Instrument) {
        if !self.shutdown {
            let m = Meter::new(name, metric_instrument);
            self.meter.push(m);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let test_meter = MeterProvider::default();
        assert_eq!(test_meter.name, env!("CARGO_PKG_NAME"));
        assert_eq!(test_meter.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(test_meter.meter.len(), 0);
        assert_eq!(test_meter.shutdown, false);
    }

    #[test]
    fn new_meter() {
        let mut global_meter = MeterProvider::default();
        assert_eq!(global_meter.meter.len(), 0);
        global_meter.new_meter(String::from("test_meter_one"), Instrument::Counter);
        assert_eq!(global_meter.meter.len(), 1);
        global_meter.new_meter(String::from("test_meter_two"), Instrument::Counter);
        assert_eq!(global_meter.meter.len(), 2);
    }
}
