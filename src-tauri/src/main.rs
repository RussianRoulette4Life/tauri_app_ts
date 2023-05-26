// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod person_logger;
use std::{sync::Mutex, io::ErrorKind, path::Path, fs, error::Error, process::exit};

use person_logger::PersonLogger;
use tauri::State;

struct PersonLoggerWrapper {
    logger: Mutex<PersonLogger>
}

const TARGET_FILE: &'static str = "../app.log";
const TARGET_FILE_DB: &'static str = "../db.sqlite3";

fn check_if_db_file_exists() -> Result<(), Box<dyn Error>> {
    let path_db = Path::new(TARGET_FILE_DB);
    match path_db.try_exists() {
        Ok(exists) => {
            if !exists {
                fs::File::create(TARGET_FILE_DB)?;
                println!("File created, everything's ok");
                // can safely unwrap cuz file exists and can execute safely cuz
                // the structure is taken from sqlite viewer lol
                let sql_table_string = fs::read_to_string("../db_structure.txt")?;
                sqlite3::open(TARGET_FILE_DB)?.execute(sql_table_string)?
            } else {
                println!("File exists, everything's ok");
            }
        },
        Err(e) => println!("Something wrong within the file system, here's an error: {e}"),
    };
    Ok(())
}
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
                ErrorKind::InvalidData => format!("<p style=\"color:\"red\"\">There are no people in the array, nothing was written.<p>"),
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
    let result = check_if_db_file_exists();
    match result {
        Ok(()) => {println!("[main::78]OK")},
        Err(n) => {
            println!("{n}");
            exit(1);
        }
    }
    tauri::Builder::default()
        .manage(PersonLoggerWrapper {logger: Mutex::new(PersonLogger::new_empty(TARGET_FILE.to_owned(), TARGET_FILE_DB.to_owned()))})
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
