use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub capital: f64,
    pub expenses: Vec<Expense>,
    pub incomes: Vec<Income>,
    pub balance_history: HashMap<u32, f64>, // age -> balance
}

impl Person {
    pub fn new(name: String, age: u32, capital: f64) -> Self {
        let mut person = Person {
            name,
            age,
            capital,
            expenses: Vec::new(),
            incomes: Vec::new(),
            balance_history: HashMap::new(),
        };
        
        // Initialize balance at current age
        person.balance_history.insert(age, capital);
        person
    }

    pub fn add_expense(&mut self, expense: Expense) {
        self.expenses.push(expense);
    }

    pub fn add_income(&mut self, income: Income) {
        self.incomes.push(income);
    }

    pub fn get_current_balance(&self) -> f64 {
        *self.balance_history.get(&self.age).unwrap_or(&0.0)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Frequency {
    #[default]
    Yearly,
    Monthly,
    Daily,
}

#[derive(Debug, Clone)]
pub struct Expense {
    pub name: String,
    pub amount: f64,
    pub frequency: Frequency, // yearly, monthly, daily
    pub start_age: u32,
    pub end_age: Option<u32>, // None means ongoing
}

impl Expense {
    pub fn new(name: String, amount: f64, frequency: Frequency, start_age: u32, end_age: Option<u32>) -> Self {
        Expense {
            name,
            amount,
            frequency,
            start_age,
            end_age,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Income {
    pub name: String,
    pub amount: f64,
    pub frequency: Frequency, // yearly, monthly, daily
    pub start_age: u32,
    pub end_age: Option<u32>, // None means ongoing
}

impl Income {
    pub fn new(name: String, amount: f64, frequency: Frequency, start_age: u32, end_age: Option<u32>) -> Self {
        Income {
            name,
            amount,
            frequency,
            start_age,
            end_age,
        }
    }
}