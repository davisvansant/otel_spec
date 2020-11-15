pub struct ValueObserver {
    pub measurement: Vec<u64>,
}

impl ValueObserver {
    pub fn default() -> ValueObserver {
        ValueObserver {
            measurement: Vec::with_capacity(50),
        }
    }

    pub async fn observe(&mut self, value: u64) {
        self.measurement.push(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let value_observer = ValueObserver::default();
        assert!(value_observer.measurement.is_empty());
    }

    #[tokio::test]
    async fn observe() {
        let mut value_observer = ValueObserver::default();
        assert!(value_observer.measurement.is_empty());

        for n in 1..=50 {
            value_observer.observe(n).await;
        }

        let min: u64 = 1;
        let max: u64 = 50;

        assert_eq!(value_observer.measurement.len(), 50);
        assert_eq!(value_observer.measurement.iter().count(), 50);
        assert_eq!(value_observer.measurement.iter().min().unwrap(), &min);
        assert_eq!(value_observer.measurement.iter().max().unwrap(), &max);
    }
}
