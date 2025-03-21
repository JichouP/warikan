pub struct Person(String);

impl Person {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_inner(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_new() {
        let person = Person::new("John".to_string());
        assert_eq!(person.as_inner(), "John");
    }
}
