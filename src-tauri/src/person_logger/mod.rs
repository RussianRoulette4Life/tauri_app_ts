use std::collections::HashMap;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use sqlite3;


///```markdown 
///# A person struct
///Should only be used within this file, made private for a reason ✨ 
///```
#[derive(Serialize, Deserialize, PartialEq)]
struct Person {
    name: String,
    middle_name: String,
    last_name: String,
    date_of_birth: String,
    comment: String,
}
#[derive(Hash, Deserialize, Serialize, PartialEq, Eq, Debug)]
struct Metadata {
    timestamp: String,
    id: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PersonLogger {
    persons: Option<HashMap<Metadata, Person>>,
    target_file: String,
    target_db: String,
}
impl Metadata {
    fn new(timestamp: String, id: i32) -> Self {
        Self {
            timestamp,
            id,
        }
    }
}
impl Person {
    pub fn to_sql(&self, metadata: &Metadata) -> String {
        let sql_string = format!("INSERT INTO students (id, name, middle_name, last_name, date_of_birth, comment, timestamp) VALUES(\"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\")", metadata.id, self.name, self.middle_name, self.last_name, self.date_of_birth, self.comment, metadata.timestamp);
        sql_string
    }
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n--------------Person Info----------------\n
               \t - Name: {};\n
               \t - Middle name: {};\n
               \t - Last name: {};\n
               \t - Date Of Birth: {};\n
               \t - Comment: {};\n-----------------------------------------\n",
               self.name, self.middle_name, self.last_name, self.date_of_birth, self.comment)
    }
}
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n--------------Person Info----------------\n
               \t - Name: {};\n
               \t - Middle name: {};\n
               \t - Last name: {};\n
               \t - Date Of Birth: {};\n
               \t - Comment: {};\n-----------------------------------------\n",
               self.name, self.middle_name, self.last_name, self.date_of_birth, self.comment)
    }
}
impl std::default::Default for Person {
    fn default() -> Self {
        Self {
            name: "John".to_owned(),
            middle_name: "Doe".to_owned(),
            last_name: "Unknowable".to_owned(),
            date_of_birth: "1.06.1970".to_owned(),
            comment: "An error must have occured somewhere right?".to_owned(),
        }
    }    
}
impl std::fmt::Display for PersonLogger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}\n
               Target file: {}\n",
               self.persons,
               self.target_file)
    }
}
impl PersonLogger {
     /// takes a json string interpretations VEC, a timestamp VEC and a target file path.
     /// returns a Result<PersonLogger, serde_json::Error>
     /// Oh and btw, this struct is actually printable, i implemented
     /// std::fmt::Display on it! les goooo
     /// (not rn tho)
     pub fn from_json_vec(persons_vec_json: Vec<String>, timestamps: Vec<String>, target_file: String, target_db: String) -> Result<Self, serde_json::Error> {
        // let persons: Vec<Person> = persons_vec_json.iter().map(
        //     |p| -> Person {
        //         match serde_json::from_str(p) {
        //             Ok(p) => p,
        //             Err(e) => {
        //                 println!("Error during parsing: {e}");
        //                 Person::default()
        //             }
        //         }
        //     }
        // ).collect();

        let mut persons_vec: Vec<Person> = vec![];
        for person_json_string in persons_vec_json {
            match serde_json::from_str::<Person>(person_json_string.as_str()) {
                Ok(p) => persons_vec.push(p),
                Err(e) => {
                    println!("An error occured!\n{e}");
                    return Err(e);
                }
            }
        }
        
        let mut persons: HashMap<Metadata, Person> = HashMap::new();
        for ( index ,person ) in persons_vec.into_iter().enumerate() {
            persons.insert(Metadata::new(timestamps.get(index).unwrap().clone() , index as i32), person);
        }
        Ok(Self {
            persons: Some(persons),
            target_file,
            target_db,
        })
     }
     ///```markdown
     ///PersonLogger::new_empty()
     ///
     /// Creates an empty PersonLogger instance, used for state management basically
     /// ```
     pub fn new_empty(target_file: String) -> Self {
        let target_db = String::new(); 
        // std::fs::File::open("../db_path.txt").unwrap().read_to_string(&mut target_db).expect("could not open file");
        Self {
            persons: None,
            target_file,
            target_db,
        }
     }
     /// ```markdown
     /// PersonLogger::append()
     /// Takes a JSON string of Person type and Metadata JSON String
     /// Appends to or replaces (if self.persons is None) array
     /// ```
     // do not touch for now i suppose cuz it works perfectly lol
     pub fn append(&mut self, persons_json: String, metadata: String) -> Result<(), serde_json::Error>{
         // println!("{}", metadata);
         // println!("{}", persons_json);
         if let Some(persons) = &mut self.persons {
            persons.insert(serde_json::from_str::<Metadata>(&metadata)?, serde_json::from_str::<Person>(&persons_json)?);
        } else {
            let mut new_person_map: HashMap<Metadata, Person> = HashMap::new();
            new_person_map.insert(serde_json::from_str::<Metadata>(&metadata)?, serde_json::from_str::<Person>(&persons_json)?);
            self.persons = Some(new_person_map);
        }
        Ok(())
     }
     ///```markdown
     ///# PersonLogger::json()
     ///
     ///returns a json string of every Person in PersonLogger::persons() hashmap
     /// 
     ///```
     ///EXPERIMENTAL RN
     pub fn json(&self) -> Option<String> {
        match &self.persons {
            Some(vec) => {
                let return_json_string: String = format!("[{}]", vec.iter().map(|(m, p)| serde_json::to_string(p).unwrap()).collect::<String>().replace("}{", "},{"));
                Some(return_json_string)
            },
            None => None,
        }
     }
     fn json_to_vec_person(&mut self, persons_vec_json: Vec<String>) -> Result<Vec<Person>, serde_json::Error> {
        let mut persons_vec: Vec<Person> = vec![];
        for person_json_string in persons_vec_json {
            match serde_json::from_str::<Person>(person_json_string.as_str()) {
                Ok(p) => persons_vec.push(p),
                Err(e) => {
                    println!("An error occured!\n{e}");
                    return Err(e);
                }
            }
        };
        Ok(persons_vec)
     }
     /// ```markdown
     ///# PersonLogger::flush()
     /// Writes the PersonLogger data to a `self.target_file` (sadly uses cloning cuz i have no
     /// idea what to do otherwise)
     /// ```
     pub fn flush(&mut self) -> std::io::Result<()> {
         if self.target_db == String::new() {
             std::fs::File::open("../db_path.txt").unwrap().read_to_string(&mut self.target_db).expect("could not open file");
         }
         // this if let Some() there is to avoid looping over nothin lol
         // if removed
         // then wont work lol
         // AFTER FLUSH THE PersonLogger.persons IS EMPTIED
         if let Some(persons) = &self.persons { 
             let mut file = OpenOptions::new()
                 .append(true)
                 .create(true)
            .open(self.target_file.clone())?;
             for (metadata, person) in persons { 
                 match write!
                     (
                         file, "[{}]Username:{}_{}_{}-Age:{}-Comment:{}\n", 
                         metadata.timestamp,
                         person.name,
                         person.middle_name,
                         person.last_name,
                         person.date_of_birth,
                         person.comment) {
                         Ok(()) => {},
                         Err(e) => println!("{:#?}", e),
                     };
                 sqlite3::open(self.target_db.clone()).unwrap().execute(person.to_sql(metadata)).expect("smth wrong with the execution of sql statement");
             }

             self.persons = None;
             Ok(())
         } else {
             Err(std::io::ErrorKind::InvalidData.into())
         }
     }
}

