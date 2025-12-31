use eframe::egui;
use crate::desktop::ui::components::shared_state::SharedState;

pub struct AnalyticsComponent;

impl AnalyticsComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut SharedState) {
        if state.simulator.is_none() {
            ui.label("Please create a person and run the simulation first.");
            return;
        }

        let simulator = state.simulator.as_ref().unwrap();
        let history = simulator.get_balance_history();

        if history.is_empty() {
            ui.label("No balance history available. Please run the simulation first.");
            return;
        }

        // Sort ages for proper chronological display
        let mut ages: Vec<u32> = history.keys().cloned().collect();
        ages.sort();

        // Create a scrollable table with analytics
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Age");
                ui.heading("Capital");
                ui.heading("Total Expenses");
                ui.heading("Total Income");
                ui.heading("Net Change");
            });

            ui.separator();

            for age in &ages {
                let balance = history.get(age).unwrap_or(&0.0);

                // Calculate total expenses and income for this age (considering frequency)
                let total_expenses: f64 = simulator.get_person().expenses
                    .iter()
                    .filter(|e| e.start_age <= *age && (e.end_age.is_none() || e.end_age.unwrap() >= *age))
                    .map(|e| {
                        // Convert to yearly amount based on frequency
                        match e.frequency {
                            crate::domain::Frequency::Yearly => e.amount,
                            crate::domain::Frequency::Monthly => e.amount * 12.0,
                            crate::domain::Frequency::Daily => e.amount * 365.0,
                        }
                    })
                    .sum();

                let total_incomes: f64 = simulator.get_person().incomes
                    .iter()
                    .filter(|i| i.start_age <= *age && (i.end_age.is_none() || i.end_age.unwrap() >= *age))
                    .map(|i| {
                        // Convert to yearly amount based on frequency
                        match i.frequency {
                            crate::domain::Frequency::Yearly => i.amount,
                            crate::domain::Frequency::Monthly => i.amount * 12.0,
                            crate::domain::Frequency::Daily => i.amount * 365.0,
                        }
                    })
                    .sum();

                // Calculate net change from previous year if available
                let prev_balance = if *age > 0 {
                    history.get(&(*age - 1)).unwrap_or(&0.0)
                } else {
                    &simulator.get_person().capital // Starting capital
                };

                let net_change = balance - prev_balance;

                ui.horizontal(|ui| {
                    ui.label(age.to_string());
                    ui.label(format!("{:.2}", balance));
                    ui.label(format!("{:.2}", total_expenses));
                    ui.label(format!("{:.2}", total_incomes));
                    ui.label(format!("{:.2}", net_change));
                });
            }
        });
    }
}