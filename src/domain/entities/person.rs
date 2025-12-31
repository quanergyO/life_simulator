use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub current_income: f64,
    pub expenses: Vec<Expense>,
    pub balance_history: HashMap<u32, f64>, // age -> balance
}

impl Person {
    pub fn new(name: String, age: u32, current_income: f64) -> Self {
        let mut person = Person {
            name,
            age,
            current_income,
            expenses: Vec::new(),
            balance_history: HashMap::new(),
        };
        
        // Initialize balance at current age
        person.balance_history.insert(age, 0.0);
        person
    }

    pub fn add_expense(&mut self, expense: Expense) {
        self.expenses.push(expense);
    }

    pub fn get_current_balance(&self) -> f64 {
        *self.balance_history.get(&self.age).unwrap_or(&0.0)
    }
}

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