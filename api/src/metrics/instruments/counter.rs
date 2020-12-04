#[derive(Clone, Debug, Default, PartialEq)]
pub struct Counter {
    pub measurement: Vec<u64>,
}

impl Counter {
    pub fn default() -> Counter {
        Counter {
            measurement: Vec::with_capacity(50),
        }
    }

    pub fn add(&mut self) {
        self.measurement.push(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let counter = Counter::default();
        assert!(counter.measurement.is_empty());
    }

    #[test]
    fn add() {
        let mut counter = Counter::default();
        assert!(counter.measurement.is_empty());
        assert_eq!(counter.measurement.iter().sum::<u64>(), 0);

        for _i in 0..50 {
            counter.add()
        }
        assert_eq!(counter.measurement.len(), 50);
        assert_eq!(counter.measurement.iter().sum::<u64>(), 50)
    }
}
