// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rand::Rng;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::api::dialog::FileDialogBuilder;
use tauri::window::Window;
use tracing::{debug, error, info};

mod cli;
mod openapi_gen;

#[tauri::command]
#[tracing::instrument]
fn get_file_path(window: Window) {
    FileDialogBuilder::new().pick_file(move |file_path| {
        let mut path_str: PathBuf = PathBuf::new();
        let path_content = match file_path {
            None => "No file picked".to_string(),
            Some(path) => match fs::read_to_string(&path) {
                Ok(contents) => {
                    debug!("File read successfully: {}", path.display());
                    path_str = path;
                    contents
                }
                Err(read_err) => {
                    error!("Unable to read file: {}", read_err);
                    path_str = path;
                    "Unable to read file".to_string()
                }
            },
        };
        let file_name = match Path::new(&path_str).file_name() {
            Some(ext) => {
                let the_thing = ext.to_str().unwrap_or("");
                debug!("Matching Some File Name: {:?}", ext);
                the_thing
            }
            None => "Unknown",
        };
        let file_type = match Path::new(&file_name).extension() {
            Some(ext) => {
                let the_thing = ext.to_str().unwrap();
                debug!("Matching Some File Type: {:?}", the_thing);
                the_thing
            }
            None => "Unknown",
        };
        window.emit("file-type", Some(file_type)).unwrap();
        window.emit("file-name", Some(file_name)).unwrap();
        window.emit("file-path", Some(path_content)).unwrap();
    })
}

#[tauri::command]
#[tracing::instrument(ret, err)]
fn my_custom_command() -> Result<String, String> {
    let mut rng = rand::thread_rng();
    debug!("This is a debug test");
    println!("I was invoked from JS!");
    info!("This is an info test");
    let test_num: u8 = rng.gen_range(1..3);
    debug!("test_num: {}", test_num);
    if test_num == 2 {
        Ok("From Rust".into())
    } else {
        Err("Something wrong".into())
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    tauri::Builder::default()
        .setup(|app| {
            if let Ok(matches) = app.get_cli_matches() {
                cli::match_cli_args(matches)
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![my_custom_command, get_file_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
