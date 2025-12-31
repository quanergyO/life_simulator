use eframe::egui;
use crate::desktop::ui::components::{
    setup_component::SetupComponent,
    expenses_component::ExpensesComponent,
    incomes_component::IncomesComponent,
    simulation_component::SimulationComponent,
    analytics_component::AnalyticsComponent,
    settings_component::SettingsComponent,
    shared_state::{SharedState, AppTab},
};

pub struct LifeSimulatorApp {
    state: SharedState,
    setup_component: SetupComponent,
    expenses_component: ExpensesComponent,
    incomes_component: IncomesComponent,
    simulation_component: SimulationComponent,
    analytics_component: AnalyticsComponent,
    settings_component: SettingsComponent,
}

impl LifeSimulatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize the appearance
        egui::Style::default();

        let mut settings_component = SettingsComponent::new();
        // Apply the initial theme
        settings_component.set_theme(settings_component.get_theme().clone(), &cc.egui_ctx);

        Self {
            state: SharedState::new(),
            setup_component: SetupComponent::new(),
            expenses_component: ExpensesComponent::new(),
            incomes_component: IncomesComponent::new(),
            simulation_component: SimulationComponent::new(),
            analytics_component: AnalyticsComponent::new(),
            settings_component,
        }
    }
}

impl eframe::App for LifeSimulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply scaling from settings component
        ctx.set_zoom_factor(self.settings_component.get_scale());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.state.current_tab, AppTab::Setup, "Setup");
                ui.selectable_value(&mut self.state.current_tab, AppTab::Expenses, "Expenses");
                ui.selectable_value(&mut self.state.current_tab, AppTab::Incomes, "Incomes");
                ui.selectable_value(&mut self.state.current_tab, AppTab::Simulation, "Simulation");

                // Only show Analytics tab if simulator has data
                if self.state.simulator.is_some() && !self.state.simulator.as_ref().unwrap().get_balance_history().is_empty() {
                    ui.selectable_value(&mut self.state.current_tab, AppTab::Analytics, "Analytics");
                }

                ui.separator();

                // Use settings component with dropdown button
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.settings_component.show_settings_button(ui, ctx);
                });
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
                },
                AppTab::Analytics => {
                    self.analytics_component.show(ui, &mut self.state);
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