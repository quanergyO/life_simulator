pub mod cli;
pub mod domain;

#[cfg(feature = "desktop")]
pub mod desktop;

#[cfg(test)]
mod tests {
    use crate::domain::{Expense, Frequency, LifeSimulator, Person};

    #[test]
    fn test_life_simulator_basic_functionality() {
        // Create a person with initial parameters
        let person = Person::new("John Doe".to_string(), 25, 50000.0);
        let mut simulator = LifeSimulator::new(person);

        // Add an expense
        let rent_expense = Expense::new("Rent".to_string(), 1000.0, Frequency::Monthly, 25, None);
        simulator.add_expense(rent_expense);

        // Calculate balance at age 30
        let balance_at_30 = simulator.calculate_balance_at_age(30);

        // Expected: 5 years of income (50000*5) - 5 years of expenses (1000*12*5)
        // = 250000 - 60000 = 190000
        assert_eq!(balance_at_30, 250000.0 - 60000.0);
    }

    #[test]
    fn test_life_simulator_with_ending_expense() {
        let person = Person::new("Jane Smith".to_string(), 30, 60000.0);
        let mut simulator = LifeSimulator::new(person);

        // Add an expense that ends at age 40
        let car_payment = Expense::new(
            "Car Payment".to_string(),
            300.0,
            Frequency::Monthly,
            30,
            Some(40),
        );
        simulator.add_expense(car_payment);

        // Calculate balance at age 45
        let balance_at_45 = simulator.calculate_balance_at_age(45);

        // Expected: 15 years of income (60000*15) - 10 years of expenses (300*12*10)
        // = 900000 - 36000 = 864000
        assert_eq!(balance_at_45, 900000.0 - 36000.0);
    }

    #[test]
    fn test_life_simulator_multiple_expenses() {
        let person = Person::new("Bob Johnson".to_string(), 20, 40000.0);
        let mut simulator = LifeSimulator::new(person);

        // Add multiple expenses
        let rent = Expense::new("Rent".to_string(), 800.0, Frequency::Monthly, 20, None);
        simulator.add_expense(rent);

        let food = Expense::new("Food".to_string(), 300.0, Frequency::Monthly, 20, None);
        simulator.add_expense(food);

        // Calculate balance at age 25
        let balance_at_25 = simulator.calculate_balance_at_age(25);

        // Expected: 5 years of income (40000*5) - 5 years of expenses ((800+300)*12*5)
        // = 200000 - 66000 = 134000
        assert_eq!(balance_at_25, 200000.0 - 66000.0);
    }

    #[test]
    fn test_life_simulator_daily_expense() {
        let person = Person::new("Alice Brown".to_string(), 22, 45000.0);
        let mut simulator = LifeSimulator::new(person);

        // Add a daily expense
        let coffee = Expense::new("Coffee".to_string(), 5.0, Frequency::Daily, 22, None);
        simulator.add_expense(coffee);

        // Calculate balance at age 23 (1 year)
        let balance_at_23 = simulator.calculate_balance_at_age(23);

        // Expected: 1 year of income (45000) - 1 year of expenses (5*365)
        // = 45000 - 1825 = 43175
        assert_eq!(balance_at_23, 45000.0 - 1825.0);
    }
}
