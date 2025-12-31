use std::io::{self, Write};

pub struct Cli {
    simulator: crate::domain::LifeSimulator,
}

impl Cli {
    pub fn new() -> Self {
        println!("Welcome to Life Simulator!");
        println!("Let's create your character.");

        let name = Self::get_input("Enter your name: ");
        let age = Self::get_input("Enter your current age: ").parse::<u32>().expect("Invalid age");
        let income = Self::get_input("Enter your current annual income: ").parse::<f64>().expect("Invalid income");

        let person = crate::domain::Person::new(name, age, income);
        let simulator = crate::domain::LifeSimulator::new(person);

        Cli { simulator }
    }

    pub fn run(&mut self) {
        loop {
            println!("\n--- Life Simulator Menu ---");
            println!("1. View current status");
            println!("2. Add expense");
            println!("3. View balance at specific age");
            println!("4. Show balance history");
            println!("5. Exit");
            print!("Choose an option: ");
            io::stdout().flush().unwrap();

            let choice = self.get_user_input().trim().parse::<u32>().unwrap_or(0);

            match choice {
                1 => self.view_current_status(),
                2 => self.add_expense(),
                3 => self.view_balance_at_age(),
                4 => self.show_balance_history(),
                5 => {
                    println!("Thanks for using Life Simulator!");
                    break;
                },
                _ => println!("Invalid option. Please try again."),
            }
        }
    }

    fn view_current_status(&self) {
        let person = self.simulator.get_person();
        println!("\n--- Current Status ---");
        println!("Name: {}", person.name);
        println!("Age: {}", person.age);
        println!("Current Income: ${:.2}", person.current_income);
        println!("Current Balance: ${:.2}", person.get_current_balance());
        println!("Number of Expenses: {}", person.expenses.len());
    }

    fn add_expense(&mut self) {
        println!("\n--- Add Expense ---");
        let name = Self::get_input("Expense name: ");
        let amount = Self::get_input("Amount per period: ").parse::<f64>().expect("Invalid amount");

        println!("Select frequency:");
        println!("1. Yearly");
        println!("2. Monthly");
        println!("3. Daily");
        print!("Choose frequency: ");
        io::stdout().flush().unwrap();

        let freq_choice = self.get_user_input().trim().parse::<u32>().unwrap_or(1);
        let frequency = match freq_choice {
            2 => crate::domain::ExpenseFrequency::Monthly,
            3 => crate::domain::ExpenseFrequency::Daily,
            _ => crate::domain::ExpenseFrequency::Yearly,
        };

        let start_age = Self::get_input("Start age for this expense: ").parse::<u32>().expect("Invalid age");
        let end_input = Self::get_input("End age for this expense (leave empty for ongoing): ");
        let end_age = if end_input.trim().is_empty() {
            None
        } else {
            Some(end_input.trim().parse::<u32>().expect("Invalid age"))
        };

        let expense = crate::domain::Expense::new(name, amount, frequency, start_age, end_age);
        self.simulator.add_expense(expense);

        println!("Expense added successfully!");
    }

    fn view_balance_at_age(&mut self) {
        let target_age_input = Self::get_input("Enter age to view balance: ");
        let target_age = target_age_input.trim().parse::<u32>().expect("Invalid age");

        let balance = self.simulator.calculate_balance_at_age(target_age);
        println!("Projected balance at age {}: ${:.2}", target_age, balance);
    }

    fn show_balance_history(&self) {
        let history = self.simulator.get_balance_history();
        if history.is_empty() {
            println!("No balance history available.");
            return;
        }

        println!("\n--- Balance History ---");
        let mut ages: Vec<u32> = history.keys().cloned().collect();
        ages.sort();

        for age in ages {
            let balance = history.get(&age).unwrap();
            println!("Age {}: ${:.2}", age, balance);
        }
    }

    fn get_user_input(&self) -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input
    }

    fn get_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }
}