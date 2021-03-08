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

    pub fn get_balance(&self) -> f64{
        self.balance
    }

    pub fn update_balance(&mut self, amount: f64) {
        self.balance += amount
    }
}