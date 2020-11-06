use std::collections::HashMap;

pub struct Baggage {
    baggage: HashMap<String, String>,
}

impl Baggage {
    pub fn init() -> Self {
        Self {
            baggage: HashMap::with_capacity(10),
        }
    }

    pub fn get_all(&self) -> &HashMap<String, String> {
        &self.baggage
    }

    pub fn get_baggage(&self, name: String) -> Option<(&String, &String)> {
        self.baggage.get_key_value(&name)
    }

    pub fn set_baggage(&mut self, name: String, value: String) -> Option<String> {
        self.baggage.insert(name, value)
    }

    pub fn remove_baggage(&mut self, name: &str) -> Option<String> {
        self.baggage.remove(name)
    }

    pub fn clear(&mut self) {
        self.baggage.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let baggage = Baggage::init();
        assert!(baggage.baggage.is_empty());
    }

    #[test]
    fn get_all() {
        let mut baggage = Baggage::init();
        baggage
            .baggage
            .insert("test".to_string(), "one".to_string());
        let get_all = baggage.get_all();
        assert!(get_all.contains_key("test"));
        assert_eq!(
            get_all.get_key_value("test"),
            Some((&"test".to_string(), &"one".to_string()))
        );
    }

    #[test]
    fn get_baggage_with_some() {
        let mut baggage = Baggage::init();
        baggage
            .baggage
            .insert("test".to_string(), "one".to_string());
        let get = baggage.get_baggage("test".to_string()).unwrap();
        assert_eq!(get.0, "test");
        assert_eq!(get.1, "one");
    }

    #[test]
    fn get_baggage_with_none() {
        let baggage = Baggage::init();
        let get = baggage.get_baggage("test".to_string());
        assert!(get.is_none());
    }

    #[test]
    fn set_baggage_with_none() {
        let mut baggage = Baggage::init();
        let set = baggage.set_baggage("test".to_string(), "one".to_string());
        let get_all = baggage.get_all();
        assert_eq!(baggage.baggage.len(), 1);
        assert!(set.is_none());
        assert!(get_all.contains_key("test"));
        assert_eq!(
            get_all.get_key_value("test"),
            Some((&"test".to_string(), &"one".to_string()))
        );
    }

    #[test]
    fn set_baggage_with_some() {
        let mut baggage = Baggage::init();
        baggage.set_baggage("test".to_string(), "one".to_string());
        let get_one = baggage.get_all();
        assert_eq!(baggage.baggage.len(), 1);
        assert!(get_one.contains_key("test"));
        assert_eq!(
            get_one.get_key_value("test"),
            Some((&"test".to_string(), &"one".to_string()))
        );
        assert_eq!(
            baggage.set_baggage("test".to_string(), "two".to_string()),
            Some("one".to_string())
        );
        let get_two = baggage.get_all();
        assert!(get_two.contains_key("test"));
        assert_eq!(
            get_two.get_key_value("test"),
            Some((&"test".to_string(), &"two".to_string()))
        );
    }

    #[test]
    fn remove_baggage_with_some() {
        let mut baggage = Baggage::init();
        baggage.set_baggage("test".to_string(), "one".to_string());
        assert_eq!(baggage.baggage.len(), 1);
        let some = baggage.remove_baggage(&"test".to_string());
        assert_eq!(baggage.baggage.len(), 0);
        assert!(some.is_some());
    }

    #[test]
    fn remove_baggage_with_none() {
        let mut baggage = Baggage::init();
        baggage.set_baggage("test".to_string(), "one".to_string());
        assert_eq!(baggage.baggage.len(), 1);
        baggage.remove_baggage(&"test".to_string());
        assert_eq!(baggage.baggage.len(), 0);
        let none = baggage.remove_baggage(&"test".to_string());
        assert!(none.is_none());
    }

    #[test]
    fn clear_baggage() {
        let mut baggage = Baggage::init();
        baggage.set_baggage("test".to_string(), "one".to_string());
        assert_eq!(baggage.baggage.len(), 1);
        baggage.clear();
        assert_eq!(baggage.baggage.len(), 0);
    }
}
