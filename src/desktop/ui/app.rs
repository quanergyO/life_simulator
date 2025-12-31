use eframe::egui;
use crate::desktop::ui::components::{
    setup_component::SetupComponent,
    expenses_component::ExpensesComponent,
    incomes_component::IncomesComponent,
    simulation_component::SimulationComponent,
    shared_state::{SharedState, AppTab},
};

// Define scaling factors
const SCALING_FACTORS: [(f32, &str); 6] = [
    (0.5, "50%"),   // 50%
    (0.75, "75%"),  // 75%
    (1.0, "100%"),  // 100%
    (1.25, "125%"), // 125%
    (1.5, "150%"),  // 150%
    (2.0, "200%"),  // 200%
];

pub struct LifeSimulatorApp {
    state: SharedState,
    setup_component: SetupComponent,
    expenses_component: ExpensesComponent,
    incomes_component: IncomesComponent,
    simulation_component: SimulationComponent,
}

impl LifeSimulatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize the appearance
        egui::Style::default();
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            state: SharedState::new(),
            setup_component: SetupComponent::new(),
            expenses_component: ExpensesComponent::new(),
            incomes_component: IncomesComponent::new(),
            simulation_component: SimulationComponent::new(),
        }
    }
}

impl eframe::App for LifeSimulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply scaling
        ctx.set_zoom_factor(self.state.current_scale);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.state.current_tab, AppTab::Setup, "Setup");
                ui.selectable_value(&mut self.state.current_tab, AppTab::Expenses, "Expenses");
                ui.selectable_value(&mut self.state.current_tab, AppTab::Incomes, "Incomes");
                ui.selectable_value(&mut self.state.current_tab, AppTab::Simulation, "Simulation");

                ui.separator();

                // Scaling controls
                ui.label("Scale:");
                for (scale_factor, label) in SCALING_FACTORS.iter() {
                    if ui.selectable_label(self.state.current_scale == *scale_factor, *label).clicked() {
                        self.state.current_scale = *scale_factor;
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.state.current_tab {
                AppTab::Setup => {
                    self.setup_component.show(ui, &mut self.state);
                },
                AppTab::Expenses => {
                    self.expenses_component.show(ui, &mut self.state);
                },
                AppTab::Incomes => {
                    self.incomes_component.show(ui, &mut self.state);
                },
                AppTab::Simulation => {
                    self.simulation_component.show(ui, &mut self.state);
                }
            }
        });
    }
}

pub fn run_gui() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::Vec2::new(1600.0, 900.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Life Simulator",
        options,
        Box::new(|cc| Box::new(LifeSimulatorApp::new(cc))),
    ).expect("Failed to run application");
}