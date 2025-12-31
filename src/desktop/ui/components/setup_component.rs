use eframe::egui;
use crate::domain::{LifeSimulator, Person};
use crate::desktop::ui::components::shared_state::SharedState;

pub struct SetupComponent {
    name: String,
    age: String,
    start_capital: String,
}

impl SetupComponent {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            age: String::new(),
            start_capital: String::new(),
        }
    }

    pub fn create_person(&mut self, state: &mut SharedState) {
        if let (Ok(age_val), Ok(income_val)) = (self.age.parse::<u32>(), self.start_capital.parse::<f64>()) {
            let person = Person::new(self.name.clone(), age_val, income_val);
            state.simulator = Some(LifeSimulator::new(person));
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut SharedState) {
        ui.heading("Life Simulator Setup");

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut self.name);
        });

        ui.horizontal(|ui| {
            ui.label("Age:");
            ui.text_edit_singleline(&mut self.age);
        });

        ui.horizontal(|ui| {
            ui.label("Current Capital:");
            ui.text_edit_singleline(&mut self.start_capital);
        });

        if ui.button("Create Person").clicked() {
            self.create_person(state);
        }

        if let Some(simulator) = &state.simulator {
            let person = simulator.get_person();
            ui.separator();
            ui.label(format!("Current Person: {}, Age: {}, Capital: {:.2}",
                             person.name, person.age, person.capital));
            ui.label(format!("Current Balance: {:.2}", person.get_current_balance()));
        }
    }
}