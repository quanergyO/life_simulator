use crate::desktop::ui::components::shared_state::SharedState;
use crate::domain::{Frequency, person::Income};
use eframe::egui;

pub struct IncomesComponent {
    income_name: String,
    income_amount: String,
    income_frequency: Frequency,
    income_start_age: String,
    income_end_age: String,
}

impl IncomesComponent {
    pub fn new() -> Self {
        Self {
            income_name: String::new(),
            income_amount: String::new(),
            income_frequency: Frequency::Yearly,
            income_start_age: String::new(),
            income_end_age: String::new(),
        }
    }

    pub fn add_income(&mut self, state: &mut SharedState) {
        if let Some(ref mut simulator) = state.simulator {
            if let (Ok(amount_val), Ok(start_age_val)) = (
                self.income_amount.parse::<f64>(),
                self.income_start_age.parse::<u32>(),
            ) {
                let end_age = if !self.income_end_age.is_empty() {
                    self.income_end_age.parse::<u32>().ok()
                } else {
                    None
                };

                let income = Income::new(
                    self.income_name.clone(),
                    amount_val,
                    self.income_frequency.clone(),
                    start_age_val,
                    end_age,
                );

                simulator.add_income(income);

                self.income_name.clear();
                self.income_amount.clear();
                self.income_start_age.clear();
                self.income_end_age.clear();
            };
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut SharedState) {
        if state.simulator.is_none() {
            ui.label("Please create a person first in the Setup tab.");
            return;
        }

        ui.heading("Add Incomes");

        ui.horizontal(|ui| {
            ui.label("Incomes Name:");
            ui.text_edit_singleline(&mut self.income_name);
        });

        ui.horizontal(|ui| {
            ui.label("Amount per period:");
            ui.text_edit_singleline(&mut self.income_amount);
        });

        ui.horizontal(|ui| {
            ui.label("Frequency:");
            ui.radio_value(&mut self.income_frequency, Frequency::Yearly, "Yearly");
            ui.radio_value(&mut self.income_frequency, Frequency::Monthly, "Monthly");
            ui.radio_value(&mut self.income_frequency, Frequency::Daily, "Daily");
        });

        ui.horizontal(|ui| {
            ui.label("Start Age:");
            ui.text_edit_singleline(&mut self.income_start_age);
        });

        ui.horizontal(|ui| {
            ui.label("End Age (optional):");
            ui.text_edit_singleline(&mut self.income_end_age);
        });

        if ui.button("Add Incomes").clicked() {
            self.add_income(state);
        }

        if let Some(simulator) = &state.simulator {
            ui.separator();
            ui.heading("Current Incomes:");
            for (i, income) in simulator.get_person().incomes.iter().enumerate() {
                ui.label(format!(
                    "{}. {}: {:.2} ({:?}) - Age {} to {}",
                    i + 1,
                    income.name,
                    income.amount,
                    income.frequency,
                    income.start_age,
                    income
                        .end_age
                        .map_or("ongoing".to_string(), |age| age.to_string())
                ));
            }
        }
    }
}
