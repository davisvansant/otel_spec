pub struct Counter {
    pub measurement: Vec<f64>,
}

impl Counter {
    pub fn default() -> Counter {
        Counter {
            measurement: Vec::with_capacity(50),
        }
    }

    pub fn add(&mut self, increment: f64) {
        self.measurement.push(increment)
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
    fn new() {
        let mut counter = Counter::default();
        assert!(counter.measurement.is_empty());
        assert_eq!(counter.measurement.iter().sum::<f64>().to_bits(), 0);

        for f in 0..50 {
            counter.add(f as f64)
        }
        assert_eq!(counter.measurement.len(), 50);
        assert!(counter.measurement.iter().sum::<f64>().is_normal());
        assert_eq!(
            counter.measurement.iter().sum::<f64>().to_bits(),
            4653102422422454272
        );
    }
}
