use eframe::egui::{self, Button, Color32, RichText};

use super::*;

pub struct LwtNgGui {
    command_tx: mpsc::Sender<Command>,
    response_rx: mpsc::Receiver<DbResult>,
    languages: Vec<db::Language>,
    new_language_to_add: String,
    last_action: RichText,
    is_initialized: bool,
}

impl LwtNgGui {
    pub fn new(
        _cc: &eframe::CreationContext<'_>,
        command_tx: mpsc::Sender<Command>,
        response_rx: mpsc::Receiver<DbResult>,
    ) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        //cc.egui_ctx.set_debug_on_hover(true);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //return eframe::get_value(storage, eframe::APP_KEY).unwrap();
        //}

        Self {
            command_tx,
            response_rx,
            languages: Vec::new(),
            new_language_to_add: String::new(),
            last_action: RichText::new(""),
            is_initialized: false,
        }
    }

    fn process_db_results(&mut self) {
        if let Ok(result) = self.response_rx.try_recv() {
            self.last_action = match result {
                crate::DbResult::AddLanguageResult => {
                    RichText::new("Language added").color(Color32::GREEN)
                }
                crate::DbResult::GetAllLanguagesResult { lang_vec } => {
                    let lang_count = lang_vec.len();
                    self.languages = lang_vec;
                    RichText::new(format!("Fetched all languages: {lang_count}"))
                        .color(Color32::GREEN)
                }
                crate::DbResult::Error { msg } => {
                    RichText::new(format!("Error: {msg}")).color(Color32::RED)
                }
            };
        }
    }

    fn init_setup(&self) {
        let tx = self.command_tx.clone();
        tokio::spawn(async move {
            tx.send(Command::GetAllLanguages).await.unwrap();
        });
    }
}

impl eframe::App for LwtNgGui {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        if !self.is_initialized {
            self.init_setup();
            self.is_initialized = true;
        }

        self.process_db_results();

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
                if ui
                    .add_enabled(!self.new_language_to_add.is_empty(), Button::new("Add"))
                    .clicked()
                {
                    let new_lang = self.new_language_to_add.clone();
                    let tx = self.command_tx.clone();
                    tokio::spawn(async move {
                        // send add and get to update the list
                        tx.send(Command::AddLanguage { name: new_lang })
                            .await
                            .unwrap();
                        tx.send(Command::GetAllLanguages).await.unwrap();
                    });
                    self.new_language_to_add.clear();
                }
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

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(self.last_action.clone());
            })
        });
    }
}
