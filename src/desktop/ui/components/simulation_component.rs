use eframe::egui;
use egui_plot::{Line, PlotPoints, Plot, Legend, VLine};
use crate::desktop::ui::components::shared_state::SharedState;

pub struct SimulationComponent {
    target_age: String,
}

impl SimulationComponent {
    pub fn new() -> Self {
        Self {
            target_age: String::new(),
        }
    }

    pub fn calculate_balance(&mut self, state: &mut SharedState) -> Option<f64> {
        if let (Some(ref mut simulator), Ok(target_age_val)) = (state.simulator.as_mut(), self.target_age.parse::<u32>()) {
            Some(simulator.calculate_balance_at_age(target_age_val))
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

        ui.horizontal(|ui| {
            ui.label("Target Age:");
            ui.text_edit_singleline(&mut self.target_age);
        });

        if ui.button("Calculate Balance").clicked() {
            if let Some(balance) = self.calculate_balance(state) {
                ui.label(format!("Projected balance: {:.2}", balance));
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
                            // Add the line to the plot
                            plot_ui.line(line);

                            // Add vertical line for current age if available
                            let current_age = simulator.get_person().age as f64;
                            plot_ui.vline(VLine::new(current_age).name("Current Age"));
                        });
                });

                // Show table of values
                ui.separator();
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.columns(2, |columns| {
                        columns[0].label("Age");
                        columns[1].label("Balance");

                        for age in &ages {
                            let balance = history.get(age).unwrap();
                            columns[0].label(age.to_string());
                            columns[1].label(format!("{:.2}", balance));
                        }
                    });
                });
            } else {
                ui.label("No balance history yet. Calculate some balances first.");
            }
        }
    }
}