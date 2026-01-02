use crate::desktop::ui::components::shared_state::SharedState;
use crate::domain::{Expense, Frequency};
use eframe::egui;

pub struct ExpensesComponent {
    expense_name: String,
    expense_amount: String,
    expense_frequency: Frequency,
    expense_start_age: String,
    expense_end_age: String,
}

impl ExpensesComponent {
    pub fn new() -> Self {
        Self {
            expense_name: String::new(),
            expense_amount: String::new(),
            expense_frequency: Frequency::Yearly,
            expense_start_age: String::new(),
            expense_end_age: String::new(),
        }
    }

    pub fn add_expense(&mut self, state: &mut SharedState) {
        if let Some(ref mut simulator) = state.simulator {
            if let (Ok(amount_val), Ok(start_age_val)) = (
                self.expense_amount.parse::<f64>(),
                self.expense_start_age.parse::<u32>(),
            ) {
                let end_age = if !self.expense_end_age.is_empty() {
                    self.expense_end_age.parse::<u32>().ok()
                } else {
                    None
                };

                let expense = Expense::new(
                    self.expense_name.clone(),
                    amount_val,
                    self.expense_frequency.clone(),
                    start_age_val,
                    end_age,
                );

                simulator.add_expense(expense);

                // Clear the input fields
                self.expense_name.clear();
                self.expense_amount.clear();
                self.expense_start_age.clear();
                self.expense_end_age.clear();
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut SharedState) {
        if state.simulator.is_none() {
            ui.label("Please create a person first in the Setup tab.");
            return;
        }

        ui.heading("Add Expenses");

        ui.horizontal(|ui| {
            ui.label("Expense Name:");
            ui.text_edit_singleline(&mut self.expense_name);
        });

        ui.horizontal(|ui| {
            ui.label("Amount per period:");
            ui.text_edit_singleline(&mut self.expense_amount);
        });

        ui.horizontal(|ui| {
            ui.label("Frequency:");
            ui.radio_value(&mut self.expense_frequency, Frequency::Yearly, "Yearly");
            ui.radio_value(&mut self.expense_frequency, Frequency::Monthly, "Monthly");
            ui.radio_value(&mut self.expense_frequency, Frequency::Daily, "Daily");
        });

        ui.horizontal(|ui| {
            ui.label("Start Age:");
            ui.text_edit_singleline(&mut self.expense_start_age);
        });

        ui.horizontal(|ui| {
            ui.label("End Age (optional):");
            ui.text_edit_singleline(&mut self.expense_end_age);
        });

        if ui.button("Add Expense").clicked() {
            self.add_expense(state);
        }

        // Show existing expenses
        if let Some(simulator) = &state.simulator {
            ui.separator();
            ui.heading("Current Expenses:");
            for (i, expense) in simulator.get_person().expenses.iter().enumerate() {
                ui.label(format!(
                    "{}. {}: {:.2} ({:?}) - Age {} to {}",
                    i + 1,
                    expense.name,
                    expense.amount,
                    expense.frequency,
                    expense.start_age,
                    expense
                        .end_age
                        .map_or("ongoing".to_string(), |age| age.to_string())
                ));
            }
        }
    }
}
