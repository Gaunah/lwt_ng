use eframe::egui::{self};

impl eframe::App for super::LwtNgGui {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            // Display an input field for the new language name
            ui.horizontal(|ui| {
                ui.label("New Language: ");
                ui.text_edit_singleline(&mut self.new_language_to_add);
                if ui.button("Add").clicked() {}
            });

            ui.vertical_centered_justified(|ui| {
                // Display the list of languages
                ui.group(|ui| {
                    ui.label("Languages:");
                    if self.languages.is_empty() {
                        ui.spinner();
                    }
                    for language in &self.languages {
                        ui.label(language.name());
                    }
                });
            });
        });
    }
}
