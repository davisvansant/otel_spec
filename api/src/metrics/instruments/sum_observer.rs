pub struct SumObserver {
    pub measurement: Vec<u64>,
}

impl SumObserver {
    pub fn default() -> SumObserver {
        SumObserver {
            measurement: Vec::with_capacity(50),
        }
    }

    pub async fn observe(&mut self) {
        self.measurement.push(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let sum_observer = SumObserver::default();
        assert!(sum_observer.measurement.is_empty());
    }

    #[tokio::test]
    async fn observe() {
        let mut sum_observer = SumObserver::default();
        assert!(sum_observer.measurement.is_empty());
        assert_eq!(sum_observer.measurement.iter().sum::<u64>(), 0);

        for _i in 0..50 {
            sum_observer.observe().await
        }
        assert_eq!(sum_observer.measurement.len(), 50);
        assert_eq!(sum_observer.measurement.iter().sum::<u64>(), 50)
    }
}
