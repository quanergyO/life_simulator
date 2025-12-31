use eframe::egui;
use egui_plot::{Line, PlotPoints, Plot, Legend};
use crate::desktop::ui::components::shared_state::SharedState;

pub struct SimulationComponent;

impl SimulationComponent {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_balance(&mut self, state: &mut SharedState) -> Option<f64> {
        if let Some(ref mut simulator) = state.simulator.as_mut() {
            Some(simulator.calculate_balance_at_age(100))
        } else {
            None
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut SharedState) {
        ui.heading("Simulation");

        if state.simulator.is_none() {
            ui.label("Please create a person and add expenses first.");
            return;
        }

        // Calculate balance automatically when simulator exists
        if ui.button("Calculate").clicked() {
            if let Some(balance) = self.calculate_balance(state) {
                ui.label(format!("Projected balance at age 100: {:.2}", balance));
            }
        }

        // Show balance history
        if let Some(simulator) = &state.simulator {
            ui.separator();
            ui.heading("Balance History:");

            let history = simulator.get_balance_history();
            if !history.is_empty() {
                let mut ages: Vec<u32> = history.keys().cloned().collect();
                ages.sort();

                let points: PlotPoints = ages
                    .iter()
                    .map(|&age| {
                        [age as f64, *history.get(&age).unwrap_or(&0.0)]
                    })
                    .collect();

                let line = Line::new(points).name("Balance over time");

                // Calculate actual expenses and incomes over time (considering frequency)
                let mut expense_points = Vec::new();
                let mut income_points = Vec::new();

                for &age in &ages {
                    // Calculate actual yearly expenses at this age
                    let total_yearly_expenses: f64 = simulator.get_person().expenses
                        .iter()
                        .filter(|e| e.start_age <= age && (e.end_age.is_none() || e.end_age.unwrap() >= age))
                        .map(|e| {
                            // Convert to yearly amount based on frequency
                            match e.frequency {
                                crate::domain::Frequency::Yearly => e.amount,
                                crate::domain::Frequency::Monthly => e.amount * 12.0,
                                crate::domain::Frequency::Daily => e.amount * 365.0,
                            }
                        })
                        .sum();

                    // Calculate actual yearly incomes at this age
                    let total_yearly_incomes: f64 = simulator.get_person().incomes
                        .iter()
                        .filter(|i| i.start_age <= age && (i.end_age.is_none() || i.end_age.unwrap() >= age))
                        .map(|i| {
                            // Convert to yearly amount based on frequency
                            match i.frequency {
                                crate::domain::Frequency::Yearly => i.amount,
                                crate::domain::Frequency::Monthly => i.amount * 12.0,
                                crate::domain::Frequency::Daily => i.amount * 365.0,
                            }
                        })
                        .sum();

                    expense_points.push([age as f64, total_yearly_expenses]);
                    income_points.push([age as f64, total_yearly_incomes]);
                }

                let expense_plot_points: PlotPoints = expense_points.into();
                let income_plot_points: PlotPoints = income_points.into();

                let expense_line = Line::new(expense_plot_points)
                    .name("Total Expenses")
                    .color(egui::Color32::RED)
                    .style(egui_plot::LineStyle::dashed_dense());

                let income_line = Line::new(income_plot_points)
                    .name("Total Income")
                    .color(egui::Color32::GREEN)
                    .style(egui_plot::LineStyle::dashed_dense());

                // Create a scroll area for the plot to allow proper sizing
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.set_height(400.0); // Set a reasonable default height

                    Plot::new("balance_plot")
                        .legend(Legend::default())
                        .view_aspect(3.0)  // Wider aspect ratio
                        .show_x(true)
                        .show_y(true)
                        .x_axis_formatter(|value, _range, _digits| format!("{}", value as u32))
                        .y_axis_formatter(|value, _range, _digits| format!("{:.2}", value))
                        .label_formatter(|name, value| format!("{}: ({:.0}, {:.2})", name, value.x, value.y))
                        .allow_zoom([true, true])  // Allow zooming on both axes
                        .allow_drag([true, true])  // Allow dragging on both axes
                        .show(ui, |plot_ui| {
                            // Add the main balance line (Capital over time)
                            plot_ui.line(line);

                            // Add the expense line
                            plot_ui.line(expense_line);

                            // Add the income line
                            plot_ui.line(income_line);
                        });
                });
            } else {
                ui.label("No balance history yet. Calculate some balances first.");
            }
        }
    }
}