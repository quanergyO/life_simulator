use eframe::egui;

// Define scaling factors
const SCALING_FACTORS: [(f32, &str); 6] = [
    (0.5, "50%"),   // 50%
    (0.75, "75%"),  // 75%
    (1.0, "100%"),  // 100%
    (1.25, "125%"), // 125%
    (1.5, "150%"),  // 150%
    (2.0, "200%"),  // 200%
];

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Dark,
    Light,
    System,
}

pub struct SettingsComponent {
    current_scale: f32,
    current_theme: Theme,
    is_open: bool,
}

impl SettingsComponent {
    pub fn new() -> Self {
        Self {
            current_scale: 1.0,         // Default to 100% scale
            current_theme: Theme::Dark, // Default to dark theme
            is_open: false,
        }
    }

    pub fn get_scale(&self) -> f32 {
        self.current_scale
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.current_scale = scale;
    }

    pub fn get_theme(&self) -> &Theme {
        &self.current_theme
    }

    pub fn set_theme(&mut self, theme: Theme, ctx: &egui::Context) {
        self.current_theme = theme.clone();

        match theme {
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::System => {
                // For now, default to dark theme in system mode
                // In a real application, you might want to detect the system theme
                ctx.set_visuals(egui::Visuals::dark());
            }
        }
    }

    pub fn show_settings_button(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if ui.button("⚙ Settings").clicked() {
            self.is_open = !self.is_open;
        }

        if self.is_open {
            // Create a popup window for settings
            let window = egui::Window::new("Settings")
                .anchor(egui::Align2::RIGHT_TOP, [0.0, 25.0])
                .collapsible(false)
                .resizable(false)
                .default_width(250.0);

            window.show(ui.ctx(), |ui| {
                ui.heading("Settings");

                ui.horizontal(|ui| {
                    ui.label("Scale:");
                    for (scale_factor, label) in SCALING_FACTORS.iter() {
                        if ui
                            .selectable_label(self.current_scale == *scale_factor, *label)
                            .clicked()
                        {
                            self.current_scale = *scale_factor;
                        }
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Visuals:");
                    let old_theme = self.current_theme.clone();
                    ui.radio_value(&mut self.current_theme, Theme::Dark, "Dark");
                    ui.radio_value(&mut self.current_theme, Theme::Light, "Light");
                    ui.radio_value(&mut self.current_theme, Theme::System, "System");

                    // Apply theme if it changed
                    if self.current_theme != old_theme {
                        self.set_theme(self.current_theme.clone(), ctx);
                    }
                });

                if ui.button("Close").clicked() {
                    self.is_open = false;
                }
            });
        }
    }
}
