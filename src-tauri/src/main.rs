// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod person_logger;
use std::{sync::Mutex, io::ErrorKind};

use person_logger::PersonLogger;
use tauri::State;

struct PersonLoggerWrapper {
    logger: Mutex<PersonLogger>
}

const TARGET_FILE: &'static str = "../app.log";
// so basically a little bit of context before i forget
// this is a test to see how can i efficiently write to files and stuff
// so for now i wanna see the frontend send a request here, serialize the answer into an object and
// then write this object somewhere on disk (preferably in the file dir)
// so here i basically get a req from the front and write the file
#[tauri::command]
fn accept_person_data(person_json_string: String, metadata: String, logger: State<PersonLoggerWrapper>) -> String{
    match logger.logger.lock().unwrap().append(person_json_string, metadata) {
        Ok(()) => (),
        Err(e) => {
            return format!("An error occured! Contact the devs with the following:\n-------ERRINFO-------\n\t- {:#?}\n-----ENDERRINFO-----", e)
        }
    };
    format!("{}", logger.logger.lock().unwrap())
}

#[tauri::command]
fn flush_logger(logger: State<PersonLoggerWrapper>) -> String {
    match logger.logger.lock().unwrap().flush() {
        Ok(()) => format!("File successfully written to {}", TARGET_FILE),
        Err(e) => {
            match e.kind() {
                ErrorKind::InvalidData => format!("<p style=\"color=\"red\"\"\">There are no people in the array, nothing was written.<p>"),
                _ => format!("Some kind of file system error occured! Error dump: {}", e.to_string()),
            }
        }
    }
}

#[tauri::command]
fn json(logger: State<PersonLoggerWrapper>) -> String {
    println!("json() called!");
    match logger.logger.lock().unwrap().json() {
        Some(s) => return s,
        None => return "nothin to show here... the list is empty mate".to_owned(),
    };
}
// TODO complete writing this one (like actually write to the file!()
// using the OpenOptions struct)
fn main() {
    tauri::Builder::default()
        .manage(PersonLoggerWrapper {logger: Mutex::new(PersonLogger::new_empty(TARGET_FILE.to_owned()))})
        .invoke_handler(tauri::generate_handler![accept_person_data, flush_logger, json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[cfg(test)]
mod tests {
    #[test]
    fn uh_huh() {
        let smth = 4;
        assert_eq!(smth, 4);
    }
}
