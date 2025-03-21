pub struct Money(u32);

impl Money {
    pub fn new(amount: u32) -> Self {
        Self(amount)
    }

    pub fn as_inner(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money_new() {
        let amount = 100;
        let money = Money::new(amount);
        assert_eq!(money.as_inner(), amount);
    }
}
