pub struct Exporter {
    pub some_collection: Vec<String>,
}

impl Exporter {
    pub fn init() -> Exporter {
        Exporter {
            some_collection: Vec::with_capacity(10),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let test_exporter = Exporter::init();
        assert_eq!(test_exporter.some_collection.len(), 0);
    }
}
