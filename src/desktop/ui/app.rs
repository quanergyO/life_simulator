use eframe::egui;
use crate::domain::{Person, Expense, ExpenseFrequency, LifeSimulator};

pub struct LifeSimulatorApp {
    simulator: Option<LifeSimulator>,
    name: String,
    age: String,
    income: String,
    expense_name: String,
    expense_amount: String,
    expense_frequency: ExpenseFrequency,
    expense_start_age: String,
    expense_end_age: String,
    target_age: String,
    current_tab: AppTab,
}

#[derive(Default, PartialEq)]
enum AppTab {
    #[default]
    Setup,
    Expenses,
    Simulation,
}

impl LifeSimulatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize the appearance
        egui::Style::default();
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        Self {
            simulator: None,
            name: String::new(),
            age: String::new(),
            income: String::new(),
            expense_name: String::new(),
            expense_amount: String::new(),
            expense_frequency: ExpenseFrequency::Yearly,
            expense_start_age: String::new(),
            expense_end_age: String::new(),
            target_age: String::new(),
            current_tab: AppTab::default(),
        }
    }

    fn create_person(&mut self) {
        if let (Ok(age_val), Ok(income_val)) = (self.age.parse::<u32>(), self.income.parse::<f64>()) {
            let person = Person::new(self.name.clone(), age_val, income_val);
            self.simulator = Some(LifeSimulator::new(person));
        }
    }

    fn add_expense(&mut self) {
        if let Some(ref mut simulator) = self.simulator {
            if let (Ok(amount_val), Ok(start_age_val)) = 
                (self.expense_amount.parse::<f64>(), self.expense_start_age.parse::<u32>()) {
                
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

    fn calculate_balance(&mut self) -> Option<f64> {
        if let (Some(ref mut simulator), Ok(target_age_val)) = (self.simulator.as_mut(), self.target_age.parse::<u32>()) {
            Some(simulator.calculate_balance_at_age(target_age_val))
        } else {
            None
        }
    }
}

impl eframe::App for LifeSimulatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_tab, AppTab::Setup, "Setup");
                ui.selectable_value(&mut self.current_tab, AppTab::Expenses, "Expenses");
                ui.selectable_value(&mut self.current_tab, AppTab::Simulation, "Simulation");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.current_tab {
                AppTab::Setup => {
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
                        ui.label("Annual Income:");
                        ui.text_edit_singleline(&mut self.income);
                    });
                    
                    if ui.button("Create Person").clicked() {
                        self.create_person();
                    }
                    
                    if let Some(simulator) = &self.simulator {
                        let person = simulator.get_person();
                        ui.separator();
                        ui.label(format!("Current Person: {}, Age: {}, Income: ${:.2}", 
                                         person.name, person.age, person.current_income));
                        ui.label(format!("Current Balance: ${:.2}", person.get_current_balance()));
                    }
                },
                
                AppTab::Expenses => {
                    ui.heading("Add Expenses");
                    
                    if self.simulator.is_none() {
                        ui.label("Please create a person first in the Setup tab.");
                        return;
                    }
                    
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
                        ui.radio_value(&mut self.expense_frequency, ExpenseFrequency::Yearly, "Yearly");
                        ui.radio_value(&mut self.expense_frequency, ExpenseFrequency::Monthly, "Monthly");
                        ui.radio_value(&mut self.expense_frequency, ExpenseFrequency::Daily, "Daily");
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
                        self.add_expense();
                    }
                    
                    // Show existing expenses
                    if let Some(simulator) = &self.simulator {
                        ui.separator();
                        ui.heading("Current Expenses:");
                        for (i, expense) in simulator.get_person().expenses.iter().enumerate() {
                            ui.label(format!("{}. {}: ${:.2} ({:?}) - Age {} to {}", 
                                           i + 1, 
                                           expense.name, 
                                           expense.amount, 
                                           expense.frequency, 
                                           expense.start_age,
                                           expense.end_age.map_or("ongoing".to_string(), |age| age.to_string())));
                        }
                    }
                },
                
                AppTab::Simulation => {
                    ui.heading("Simulation");
                    
                    if self.simulator.is_none() {
                        ui.label("Please create a person and add expenses first.");
                        return;
                    }
                    
                    ui.horizontal(|ui| {
                        ui.label("Target Age:");
                        ui.text_edit_singleline(&mut self.target_age);
                    });
                    
                    if ui.button("Calculate Balance").clicked() {
                        if let Some(balance) = self.calculate_balance() {
                            ui.label(format!("Projected balance: ${:.2}", balance));
                        }
                    }
                    
                    // Show balance history
                    if let Some(simulator) = &self.simulator {
                        ui.separator();
                        ui.heading("Balance History:");
                        
                        let history = simulator.get_balance_history();
                        if !history.is_empty() {
                            use egui_plot::{Line, PlotPoints, Plot};
                            
                            let mut ages: Vec<u32> = history.keys().cloned().collect();
                            ages.sort();
                            
                            let points: PlotPoints = ages
                                .iter()
                                .map(|&age| {
                                    [age as f64, *history.get(&age).unwrap_or(&0.0)]
                                })
                                .collect();
                            
                            let line = Line::new(points).name("Balance over time");
                            
                            Plot::new("balance_plot")
                                .view_aspect(2.0)
                                .show_x(true)
                                .show_y(true)
                                .label_formatter(|name, value| format!("{}: ({:.0}, ${:.2})", name, value.x, value.y))
                                .show(ui, |plot_ui| {
                                    plot_ui.line(line);
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
                                        columns[1].label(format!("${:.2}", balance));
                                    }
                                });
                            });
                        } else {
                            ui.label("No balance history yet. Calculate some balances first.");
                        }
                    }
                }
            }
        });
    }
}

pub fn run_gui() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::Vec2::new(800.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Life Simulator",
        options,
        Box::new(|cc| Box::new(LifeSimulatorApp::new(cc))),
    ).expect("Failed to run application");
}