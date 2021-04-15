use super::Increment;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UpDownSumObserver {
    pub measurement: Vec<i64>,
}

impl UpDownSumObserver {
    pub fn default() -> UpDownSumObserver {
        UpDownSumObserver {
            measurement: Vec::with_capacity(50),
        }
    }

    pub async fn observe(&mut self, sum: Increment) {
        if sum == Increment::Up {
            self.measurement.push(1_i64)
        } else {
            self.measurement.push(-1_i64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let up_down_sum_obvserer = UpDownSumObserver::default();
        assert!(up_down_sum_obvserer.measurement.is_empty());
    }

    #[tokio::test]
    async fn observe_positive() {
        let mut up_down_sum_obvserer = UpDownSumObserver::default();
        assert!(up_down_sum_obvserer.measurement.is_empty());

        up_down_sum_obvserer.observe(Increment::Up).await;
        up_down_sum_obvserer.observe(Increment::Up).await;
        up_down_sum_obvserer.observe(Increment::Up).await;
        up_down_sum_obvserer.observe(Increment::Up).await;
        up_down_sum_obvserer.observe(Increment::Up).await;
        up_down_sum_obvserer.observe(Increment::Down).await;
        up_down_sum_obvserer.observe(Increment::Down).await;
        up_down_sum_obvserer.observe(Increment::Down).await;

        assert_eq!(up_down_sum_obvserer.measurement.len(), 8);
        assert_eq!(up_down_sum_obvserer.measurement.iter().sum::<i64>(), 2);
    }

    #[tokio::test]
    async fn observe_negative() {
        let mut up_down_sum_obvserer = UpDownSumObserver::default();
        assert!(up_down_sum_obvserer.measurement.is_empty());

        up_down_sum_obvserer.observe(Increment::Up).await;
        up_down_sum_obvserer.observe(Increment::Up).await;
        up_down_sum_obvserer.observe(Increment::Up).await;
        up_down_sum_obvserer.observe(Increment::Down).await;
        up_down_sum_obvserer.observe(Increment::Down).await;
        up_down_sum_obvserer.observe(Increment::Down).await;
        up_down_sum_obvserer.observe(Increment::Down).await;
        up_down_sum_obvserer.observe(Increment::Down).await;

        assert_eq!(up_down_sum_obvserer.measurement.len(), 8);
        assert_eq!(up_down_sum_obvserer.measurement.iter().sum::<i64>(), -2);
    }
}
