use crate::domain::{
    entities::person::{Expense, Frequency, Person},
    person::Income,
};

pub struct LifeSimulator {
    person: Person,
}

impl LifeSimulator {
    pub fn new(person: Person) -> Self {
        LifeSimulator { person }
    }

    pub fn add_expense(&mut self, expense: Expense) {
        self.person.add_expense(expense);
    }

    pub fn add_income(&mut self, income: Income) {
        self.person.add_income(income);
    }

    pub fn calculate_balance_at_age(&mut self, target_age: u32) -> f64 {
        if target_age == self.person.age {
            return self.person.get_current_balance();
        }

        // If we already calculated this age, return the stored value
        if let Some(cached_balance) = self.person.balance_history.get(&target_age) {
            return *cached_balance;
        }

        // Calculate balance from the closest known age
        let balance = self.calculate_balance_progression(target_age);

        // Update the person's age and store the balance
        self.person.balance_history.insert(target_age, balance);

        balance
    }

    fn calculate_balance_progression(&mut self, target_age: u32) -> f64 {
        // Find the closest age that we have a balance for
        let mut current_age = self.person.age;
        let mut current_balance = self.person.get_current_balance();

        // Determine direction of calculation
        if target_age > current_age {
            // Calculate forward in time
            while current_age < target_age {
                current_balance = self.calculate_balance_for_year(current_age, current_balance);
                current_age += 1;

                // Store intermediate balances for efficiency
                self.person
                    .balance_history
                    .insert(current_age, current_balance);
            }
        } else {
            // Calculate backward in time (simplified - in a real app, this would require more complex logic)
            // For now, we'll just return the current balance for past ages if not available
            if let Some(balance) = self.person.balance_history.get(&target_age) {
                return *balance;
            }
            // If we don't have historical data, return current balance as a fallback
            return current_balance;
        }

        current_balance
    }

    fn calculate_balance_for_year(&self, current_age: u32, current_balance: f64) -> f64 {
        let mut yearly_expenses = 0.0;
        let mut yearly_incomes = 0.0;

        for expense in &self.person.expenses {
            if current_age >= expense.start_age {
                if let Some(end_age) = expense.end_age {
                    if current_age >= end_age {
                        continue; // Expense has ended
                    }
                }

                // Calculate yearly expense based on frequency
                let yearly_expense_amount = match expense.frequency {
                    Frequency::Yearly => expense.amount,
                    Frequency::Monthly => expense.amount * 12.0,
                    Frequency::Daily => expense.amount * 365.0,
                };

                yearly_expenses += yearly_expense_amount;
            }
        }

        for income in &self.person.incomes {
            if current_age >= income.start_age {
                if let Some(end_age) = income.end_age {
                    if current_age >= end_age {
                        continue; // Expense has ended
                    }
                }
            }

            let yearly_income_amount = match income.frequency {
                Frequency::Yearly => income.amount,
                Frequency::Monthly => income.amount * 12.0,
                Frequency::Daily => income.amount * 365.0,
            };

            yearly_incomes += yearly_income_amount;
        }

        // Add income for the year and subtract expenses
        current_balance - yearly_expenses + yearly_incomes
    }

    pub fn get_person(&self) -> &Person {
        &self.person
    }

    pub fn get_person_mut(&mut self) -> &mut Person {
        &mut self.person
    }

    pub fn get_balance_history(&self) -> &std::collections::HashMap<u32, f64> {
        &self.person.balance_history
    }

    pub fn get_current_age(&self) -> u32 {
        self.person.age
    }

    pub fn set_current_age(&mut self, age: u32) {
        self.person.age = age;
    }
}
