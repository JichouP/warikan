use super::{money::Money, person::Person};

pub struct Repayment {
    money: Money,
    from: Person,
    to: Person,
}

impl Repayment {
    pub fn new(money: Money, from: Person, to: Person) -> Self {
        Self { money, from, to }
    }

    pub fn money(&self) -> &Money {
        &self.money
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
    fn test_repayment_new() {
        let amount = 100;
        let money = Money::new(amount);
        let from = Person::new("John".to_string());
        let to = Person::new("Jane".to_string());
        let repayment = Repayment::new(money.clone(), from.clone(), to.clone());

        assert_eq!(repayment.money(), &money);
        assert_eq!(repayment.from(), &from);
        assert_eq!(repayment.to(), &to);
    }
}
