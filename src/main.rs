#[allow(unused)]
mod db;
mod gui;
use std::env;

use sqlx::{Pool, Sqlite};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct LwtNgGui {
    #[serde(skip)]
    pool: Option<Pool<Sqlite>>,
    #[serde(skip)]
    pub languages: Vec<db::Language>,
    pub new_language_to_add: String,
}

impl LwtNgGui {
    fn new(cc: &eframe::CreationContext<'_>, pool: Pool<Sqlite>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        //cc.egui_ctx.set_debug_on_hover(true);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //return eframe::get_value(storage, eframe::APP_KEY).unwrap();
        //}

        Self {
            pool: Some(pool),
            languages: Vec::new(),
            new_language_to_add: String::new(),
        }
    }
}

#[tokio::main]
async fn main() -> eframe::Result<()> {
    dotenvy::dotenv().unwrap();
    let db_url = env!("DATABASE_URL");
    let pool = db::setup_db_connection(db_url).await.unwrap();

    let native_options = eframe::NativeOptions {
        follow_system_theme: true,
        ..Default::default()
    };

    eframe::run_native(
        "LWT_NG",
        native_options,
        Box::new(|cc| Box::new(LwtNgGui::new(cc, pool))),
    )
}
