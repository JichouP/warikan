use super::{money::Money, person::Person};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payment {
    amount: Money,
    from: Person,
    to: Person,
}

impl Payment {
    pub fn new(amount: Money, from: Person, to: Person) -> Self {
        Self { amount, from, to }
    }

    pub fn amount(&self) -> &Money {
        &self.amount
    }

    pub fn from(&self) -> &Person {
        &self.from
    }

    pub fn to(&self) -> &Person {
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
        let to = Person::new("Jane".to_string());
        let payment = Payment::new(money, from.clone(), to.clone());

        assert_eq!(payment.amount().as_inner(), amount);
        assert_eq!(payment.from(), &from);
        assert_eq!(payment.to(), &to);
    }
}
