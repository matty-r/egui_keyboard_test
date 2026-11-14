/// We derive Deserialize/Serialize so we can persist app state on shutdown.

pub struct TestApp {
    normal_input: String,
    numeric_input: String,
    decimal_input: String,
    telephone_input: String,
    email_input: String,
    url_input: String,
    password_input: String,
}

impl Default for TestApp {
    fn default() -> Self {
        Self {
            normal_input: "".to_string(),
            numeric_input: "".to_string(),
            decimal_input: "".to_string(),
            telephone_input: "".to_string(),
            email_input: "".to_string(),
            url_input: "".to_string(),
            password_input: "".to_string(),
        }
    }
}

impl TestApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TestApp {

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Panel::top("top_panel").show(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Virtual Keyboard Type Test");

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Normal input: ");
                    ui.add(egui::TextEdit::singleline(&mut self.normal_input));
                });
                ui.small("Note: No 'special' input type.");

                ui.horizontal(|ui| {
                    ui.label("Numeric input: ");
                    ui.add(egui::TextEdit::singleline(&mut self.numeric_input).keyboard_type(egui::KeyboardType::Numeric));
                });
                ui.small("Note: Numeric input keyboard, but only requires the digits 0-9");

                ui.horizontal(|ui| {
                    ui.label("Decimal input: ");
                    ui.add(egui::TextEdit::singleline(&mut self.decimal_input).keyboard_type(egui::KeyboardType::Decimal));
                });
                ui.small("Note: Fractional numeric input keyboard containing the digits and decimal separator");

                ui.horizontal(|ui| {
                    ui.label("Telephone input: ");
                    ui.add(egui::TextEdit::singleline(&mut self.telephone_input).keyboard_type(egui::KeyboardType::Telephone));
                });
                ui.small("Note: A telephone keypad input, including the digits 0-9, the asterisk (*), and the pound (#) key");

                ui.horizontal(|ui| {
                    ui.label("Email input: ");
                    ui.add(egui::TextEdit::singleline(&mut self.email_input).keyboard_type(egui::KeyboardType::Email));
                });
                ui.small("Note: A virtual keyboard optimized for entering email addresses");

                ui.horizontal(|ui| {
                    ui.label("URL input: ");
                    ui.add(egui::TextEdit::singleline(&mut self.url_input).keyboard_type(egui::KeyboardType::Url));
                });
                ui.small("Note: A keypad optimized for entering URLs");

                ui.horizontal(|ui| {
                    ui.label("Password input: ");
                    ui.add(egui::TextEdit::singleline(&mut self.password_input).password(true));
                });
                ui.small("Note: In addition to the normal egui/eframe handling of password fields, features like autocapitalize/autocomplete will be disabled")
            });

            ui.separator();
        });
    }
}