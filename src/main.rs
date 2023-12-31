#[allow(unused)]
mod db;
mod gui;
use std::{env, process};

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
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().unwrap_or_else(|err| {
        tracing::error!("{err}");
        process::exit(1);
    });
    let pool = db::setup_db_connection(env!("DATABASE_URL"))
        .await
        .unwrap_or_else(|err| {
            tracing::error!("{err}");
            process::exit(1);
        });

    let (command_tx, command_rx) = mpsc::channel(5);
    let (response_tx, response_rx) = mpsc::channel(5);

    let manager = spawn_command_manager(command_rx, response_tx, pool);

    let native_options = eframe::NativeOptions {
        follow_system_theme: true,
        ..Default::default()
    };

    eframe::run_native(
        "LWT_NG",
        native_options,
        Box::new(|cc| Box::new(gui::LwtNgGui::new(cc, command_tx, response_rx))),
    )
    .unwrap_or_else(|err| {
        tracing::error!("{err}");
        process::exit(1);
    });

    manager.await.unwrap_or_else(|err| {
        tracing::error!("{err}");
        process::exit(1);
    });
}

fn spawn_command_manager(
    mut command_rx: mpsc::Receiver<Command>,
    response_tx: mpsc::Sender<DbResult>,
    pool: sqlx::Pool<sqlx::Sqlite>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(cmd) = command_rx.recv().await {
            if let Err(e) = match cmd {
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
            } {
                tracing::error!("{e}");
            }
        }
    })
}
