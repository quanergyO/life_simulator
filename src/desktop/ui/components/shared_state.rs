use crate::domain::LifeSimulator;

#[derive(Default, PartialEq)]
pub enum AppTab {
    #[default]
    Setup,
    Expenses,
    Incomes,
    Simulation,
    Analytics,
}

#[derive(Default)]
pub struct SharedState {
    pub simulator: Option<LifeSimulator>,
    pub current_tab: AppTab,
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            simulator: None,
            current_tab: AppTab::default(),
        }
    }
}