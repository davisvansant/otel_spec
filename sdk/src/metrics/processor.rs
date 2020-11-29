pub struct Processor {
    pub some_collection: Vec<String>,
}

impl Processor {
    pub fn init() -> Processor {
        Processor {
            some_collection: Vec::with_capacity(10),
        }
    }

    pub fn process() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let test_processor = Processor::init();
        assert_eq!(test_processor.some_collection.len(), 0);
    }
}
