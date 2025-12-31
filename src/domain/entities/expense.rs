#[derive(Debug, Clone)]
pub struct Expense {
    pub name: String,
    pub amount: f64,
    pub frequency: ExpenseFrequency, // yearly, monthly, daily
    pub start_age: u32,
    pub end_age: Option<u32>, // None means ongoing
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpenseFrequency {
    Yearly,
    Monthly,
    Daily,
}

impl Expense {
    pub fn new(name: String, amount: f64, frequency: ExpenseFrequency, start_age: u32, end_age: Option<u32>) -> Self {
        Expense {
            name,
            amount,
            frequency,
            start_age,
            end_age,
        }
    }
}