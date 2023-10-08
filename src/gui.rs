use eframe::{
    egui::{self, Button, Color32, RichText},
    epaint::{FontId, Vec2},
};
use egui::FontFamily::Proportional;
use egui::TextStyle::*;

use super::*;

pub struct LwtNgGui {
    command_tx: mpsc::Sender<Command>,
    response_rx: mpsc::Receiver<DbResult>,
    languages: Vec<db::Language>,
    current_language: Option<db::Language>,
    new_language_to_add: String,
    last_action: RichText,
    is_initialized: bool,
}

impl LwtNgGui {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
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

        let mut style = (*cc.egui_ctx.style()).clone();

        // Redefine text_styles
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(14.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        style.spacing.item_spacing = Vec2::new(4., 4.);
        style.spacing.button_padding = Vec2::new(3., 3.);

        // Mutate global style with above changes
        cc.egui_ctx.set_style(style);

        Self {
            command_tx,
            response_rx,
            languages: Vec::new(),
            current_language: None,
            new_language_to_add: String::new(),
            last_action: RichText::new(""),
            is_initialized: false,
        }
    }

    fn process_db_results(&mut self) {
        if let Ok(result) = self.response_rx.try_recv() {
            self.last_action = match result {
                DbResult::AddLanguageResult => {
                    RichText::new("Language added").color(Color32::GREEN)
                }
                DbResult::GetAllLanguagesResult { lang_vec } => {
                    let lang_count = lang_vec.len();
                    self.languages = lang_vec;
                    RichText::new(format!("Fetched all languages: {lang_count}"))
                        .color(Color32::GREEN)
                }
                DbResult::Error { msg } => {
                    tracing::error!(msg);
                    RichText::new(format!("Error: {msg}")).color(Color32::RED)
                }
            };
        }
    }

    fn init_setup(&self) {
        let tx = self.command_tx.clone();
        tokio::spawn(async move {
            if let Err(e) = tx.send(Command::GetAllLanguages).await {
                tracing::error!("{e}");
            }
        });
    }

    fn render_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.horizontal(|ui| {
                    let library_btn = ui.add(egui::Button::new("Library"));
                    let vocab_btn = ui.add(egui::Button::new("Vocabulary"));
                });
                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    let mut lang_name = "None";
                    if let Some(lang) = &self.current_language {
                        lang_name = lang.name();
                    }
                    egui::ComboBox::from_label("Language:")
                        .selected_text(lang_name)
                        .show_ui(ui, |ui| {
                            for lang in &self.languages {
                                ui.selectable_value(
                                    &mut self.current_language,
                                    Some(lang.clone()),
                                    lang.name(),
                                );
                            }
                        });
                });
            });
        });
    }

    fn render_center_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            // Display an input field for the new language name
            ui.horizontal(|ui| {
                ui.label("New Language:");
                ui.text_edit_singleline(&mut self.new_language_to_add);
                let add_lang_btn =
                    ui.add_enabled(!self.new_language_to_add.is_empty(), Button::new("Add"));

                if add_lang_btn.clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let new_lang = self.new_language_to_add.clone();
                    let tx = self.command_tx.clone();
                    tokio::spawn(async move {
                        // send add and get to update the list
                        if let Err(e) = tx.send(Command::AddLanguage { name: new_lang }).await {
                            tracing::error!("{e}");
                        }
                        if let Err(e) = tx.send(Command::GetAllLanguages).await {
                            tracing::error!("{e}");
                        }
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

        self.render_top_panel(ctx);

        self.render_center_panel(ctx);

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(self.last_action.clone());
            })
        });
    }
}
