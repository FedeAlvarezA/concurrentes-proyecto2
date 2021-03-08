#[derive(Debug)]
pub struct User {
    id: String,
    balance: f64
}

impl User {
    pub fn new(id: String) -> User {
        User {
            id,
            balance: 0.0
        }
    }

    pub fn update_balance(&mut self, amount: f64) {
        self.balance += amount
    }
}

#[cfg(test)]
mod tests {
    use crate::user::User;

    #[test]
    fn update_balance_sets_property_correctly() {
        let mut user = User::new(String::from("test"));
        let change = 25.5;
        user.update_balance(change);
        
        assert_eq!(user.balance, change);

        user.update_balance(-15.5);

        assert_eq!(user.balance, 10.0)
    }

}