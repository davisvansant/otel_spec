use crate::metrics::meter::Meter;

pub struct MeterProvider {
    pub name: &'static str,
    pub version: &'static str,
    pub meter: Vec<Meter>,
}

impl MeterProvider {
    pub fn default() -> MeterProvider {
        MeterProvider {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            meter: Vec::with_capacity(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let global_meter = MeterProvider::default();
        assert_eq!(global_meter.meter.len(), 0);
    }
}
