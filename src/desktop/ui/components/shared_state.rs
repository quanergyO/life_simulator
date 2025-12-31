use crate::domain::LifeSimulator;

#[derive(Default, PartialEq)]
pub enum AppTab {
    #[default]
    Setup,
    Expenses,
    Incomes,
    Simulation,
}

#[derive(Default)]
pub struct SharedState {
    pub simulator: Option<LifeSimulator>,
    pub current_tab: AppTab,
    pub current_scale: f32,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            simulator: None,
            current_tab: AppTab::default(),
            current_scale: 1.0, // Default to 100% scale
        }
    }
}