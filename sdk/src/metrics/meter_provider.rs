use api::metrics::meter_provider::MeterProvider;

pub trait MeterShutdown {
    fn shutdown(&mut self);
}

impl MeterShutdown for MeterProvider {
    fn shutdown(&mut self) {
        self.shutdown = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::metrics::meter::Instrument;

    #[test]
    fn shutdown() {
        let mut test_meter = MeterProvider::default();
        assert_eq!(test_meter.meter.len(), 0);
        test_meter.new_meter(String::from("test_meter_one"), Instrument::Counter);
        assert_eq!(test_meter.meter.len(), 1);
        test_meter.shutdown();
        test_meter.new_meter(String::from("test_meter_two"), Instrument::Counter);
        assert_eq!(test_meter.meter.len(), 1);
    }
}
