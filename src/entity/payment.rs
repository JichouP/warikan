use super::{money::Money, person::Person};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    money: Money,
    from: Person,
    to: Vec<Person>,
}

impl Payment {
    pub fn new(money: Money, from: Person, to: Vec<Person>) -> Self {
        Self { money, from, to }
    }

    pub fn money(&self) -> &Money {
        &self.money
    }

    pub fn from(&self) -> &Person {
        &self.from
    }

    pub fn to(&self) -> &Vec<Person> {
        &self.to
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_new() {
        let amount = 100;
        let money = Money::new(amount);
        let from = Person::new("John".to_string());
        let to = vec![
            Person::new("Jane".to_string()),
            Person::new("Doe".to_string()),
        ];
        let payment = Payment::new(money.clone(), from.clone(), to.clone());

        assert_eq!(payment.money(), &money);
        assert_eq!(payment.from(), &from);
        assert_eq!(payment.to(), &to);
    }
}
