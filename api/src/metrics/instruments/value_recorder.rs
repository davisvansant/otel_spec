#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValueRecorder {
    pub measurement: Vec<u64>,
}

impl ValueRecorder {
    pub fn default() -> ValueRecorder {
        ValueRecorder {
            measurement: Vec::with_capacity(50),
        }
    }

    pub fn record(&mut self, value: u64) {
        self.measurement.push(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let value_recorder = ValueRecorder::default();
        assert!(value_recorder.measurement.is_empty());
    }

    #[test]
    fn record() {
        let mut value_recorder = ValueRecorder::default();
        assert!(value_recorder.measurement.is_empty());

        for n in 1..=50 {
            value_recorder.record(n);
        }

        let min: u64 = 1;
        let max: u64 = 50;

        assert_eq!(value_recorder.measurement.len(), 50);
        assert_eq!(value_recorder.measurement.iter().count(), 50);
        assert_eq!(value_recorder.measurement.iter().min().unwrap(), &min);
        assert_eq!(value_recorder.measurement.iter().max().unwrap(), &max);
    }
}
