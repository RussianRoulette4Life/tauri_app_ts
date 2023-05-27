// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod person_logger;
use std::{sync::Mutex, io::ErrorKind, path::{Path, PathBuf}, fs, error::Error, process::exit};

use person_logger::PersonLogger;
use tauri::State;

struct PersonLoggerWrapper {
    logger: Mutex<PersonLogger>
}

const TARGET_FILE: &'static str = "../app.log";
const TARGET_FILE_DB: &'static str = "databases/db.sqlite3";

fn check_if_db_file_exists(path_db: &PathBuf) -> Result<(), Box<dyn Error>> {
    println!("{:#?}", path_db); 
    match path_db.try_exists() {
        Ok(exists) => {
            if !exists {
                fs::File::create(path_db)?;
                println!("File created, everything's ok");
                // can safely unwrap cuz file exists and can execute safely cuz
                // the structure is taken from sqlite viewer lol
                let sql_table_string = fs::read_to_string("../db_structure.txt")?;
                sqlite3::open(path_db)?.execute(sql_table_string)?
            } else {
                println!("File exists, everything's ok");
            }
        },
        Err(e) => println!("Something wrong within the file system, here's an error: {e}"),
    };
    Ok(())
}
fn init_project_dirs(app_data_dir: &PathBuf) {
    let database_path = app_data_dir.join("databases");
    if !database_path.exists() {
        fs::create_dir(database_path).expect("COULD NOT CREATE DATABASE DIR");
    }
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
    println!("in app closure");
    tauri::Builder::default()
        .setup( |app| {
            let path_resolver = app.path_resolver();
            let app_data_path = path_resolver.app_data_dir().unwrap();
            init_project_dirs(&app_data_path);
            let full_db_path = app_data_path.join(TARGET_FILE_DB);
            // have to write this path to a file because closures and my stupidity
            let db_exists_result = check_if_db_file_exists(&full_db_path);
            fs::write("../db_path.txt", full_db_path.to_str().unwrap())?;
            match db_exists_result {
                Ok(()) => {println!("[main::tauri::Builder::default().setup()]OK");},
                Err(n) => {
                    println!("{n}");
                    exit(1);
                }
            }
            println!("{:#?}", path_resolver.app_data_dir());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![accept_person_data, flush_logger, json])
        .manage(PersonLoggerWrapper {logger: Mutex::new(PersonLogger::new_empty(TARGET_FILE.to_owned()))})
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
