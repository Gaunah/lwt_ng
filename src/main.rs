#[allow(unused)]
mod db;
mod gui;
use std::env;

use tokio::sync::mpsc;

/// Multiple different commands are multiplexed over a single channel.
#[derive(Debug)]
pub enum Command {
    AddLanguage { name: String },
    GetAllLanguages,
}

#[derive(Debug)]
pub enum DbResult {
    AddLanguageResult,
    GetAllLanguagesResult { lang_vec: Vec<db::Language> },
    Error { msg: String },
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let db_url = env!("DATABASE_URL");
    let pool = db::setup_db_connection(db_url)
        .await
        .expect("could not open database connection");

    let (command_tx, mut command_rx) = mpsc::channel(10);
    let (response_tx, response_rx) = mpsc::channel(10);

    let manager = tokio::spawn(async move {
        while let Some(cmd) = command_rx.recv().await {
            let _ = match cmd {
                Command::GetAllLanguages => {
                    match db::get_all_languages(&pool).await {
                        Ok(lang_vec) => {
                            response_tx.send(DbResult::GetAllLanguagesResult { lang_vec })
                        }
                        Err(e) => response_tx.send(DbResult::Error { msg: e.to_string() }),
                    }
                    .await
                }
                Command::AddLanguage { name } => {
                    match db::add_language(&name, &pool).await {
                        Ok(_res) => response_tx.send(DbResult::AddLanguageResult),
                        Err(e) => response_tx.send(DbResult::Error { msg: e.to_string() }),
                    }
                    .await
                }
            };
        }
    });

    let native_options = eframe::NativeOptions {
        follow_system_theme: true,
        ..Default::default()
    };

    eframe::run_native(
        "LWT_NG",
        native_options,
        Box::new(|cc| Box::new(gui::LwtNgGui::new(cc, command_tx, response_rx))),
    )
    .unwrap();

    manager.await.unwrap();
}
