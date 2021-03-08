pub struct User {
    id: String,
    balance: f64
}

impl User {
    pub fn new(id: String) {
        User {
            id,
            0
        }
    }

    pub fn get_balance(&mut self){
        self.balance
    }

    pub fn update_balance(&mut self, amount: f64) {
        self.balance += amount
    }
}