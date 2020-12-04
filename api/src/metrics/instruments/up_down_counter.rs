use super::Increment;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct UpDownCounter {
    pub measurement: Vec<i64>,
}

impl UpDownCounter {
    pub fn default() -> UpDownCounter {
        UpDownCounter {
            measurement: Vec::with_capacity(50),
        }
    }

    pub fn add(&mut self, increment: Increment) {
        if increment == Increment::Up {
            self.measurement.push(1 as i64)
        } else {
            self.measurement.push(-1 as i64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let up_down_counter = UpDownCounter::default();
        assert!(up_down_counter.measurement.is_empty());
    }

    #[test]
    fn add_positive() {
        let mut up_down_counter = UpDownCounter::default();
        assert!(up_down_counter.measurement.is_empty());

        up_down_counter.add(Increment::Up);
        up_down_counter.add(Increment::Up);
        up_down_counter.add(Increment::Up);
        up_down_counter.add(Increment::Up);
        up_down_counter.add(Increment::Up);
        up_down_counter.add(Increment::Down);
        up_down_counter.add(Increment::Down);
        up_down_counter.add(Increment::Down);

        assert_eq!(up_down_counter.measurement.len(), 8);
        assert_eq!(up_down_counter.measurement.iter().sum::<i64>(), 2);
    }

    #[test]
    fn add_negative() {
        let mut up_down_counter = UpDownCounter::default();
        assert!(up_down_counter.measurement.is_empty());

        up_down_counter.add(Increment::Up);
        up_down_counter.add(Increment::Up);
        up_down_counter.add(Increment::Up);
        up_down_counter.add(Increment::Down);
        up_down_counter.add(Increment::Down);
        up_down_counter.add(Increment::Down);
        up_down_counter.add(Increment::Down);
        up_down_counter.add(Increment::Down);

        assert_eq!(up_down_counter.measurement.len(), 8);
        assert_eq!(up_down_counter.measurement.iter().sum::<i64>(), -2);
    }
}
