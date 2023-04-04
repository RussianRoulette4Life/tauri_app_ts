use std::collections::HashMap;
/// basically a person type. Note: the From<T> trait is implemented
/// with a tuple of (Username(String), Age(i32), Timestamp(String), Comment(String)), IN THIS ORDER!
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{Read, Write};
use std::time::SystemTime;
use serde::{Serialize, Deserialize};

///```markdown 
///# A person struct
///Should only be used within this file, made private for a reason ✨ 
///```
#[derive(Serialize, Deserialize, PartialEq)]
struct Person {
    /// a field for a username
    username: String,
    /// a field for an age
    age: i32,
    /// a field for the comment
    comment: String,
}
#[derive(Hash, Deserialize, Serialize, PartialEq, Eq, Debug)]
struct Metadata {
    timestamp: String,
    id: usize,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PersonLogger {
    persons: Option<HashMap<Metadata, Person>>,
    target_file: String,
}
impl From<(String, i32, String)> for Person{
    fn from(value: (String, i32, String)) -> Self {
        Self {
            username: value.0,
            age: value.1,
            comment: value.2,
        }
    }
}
impl Metadata {
    fn new(timestamp: String, id: usize) -> Self {
        Self {
            timestamp,
            id,
        }
    }
}
impl From<&(String, i32, String)> for Person{
    fn from(value: &(String, i32, String)) -> Self {
        Self {
            username: value.0.clone(),
            age: value.1,
            comment: value.2.clone(),
        }
    }
}
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n--------------Person Info----------------\n
               \t - Username: {};\n
               \t - Age: {};\n
               \t - comment: {};\n-----------------------------------------\n",
               self.username, self.age, self.comment)
    }
}
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n--------------Person Info----------------\n
               \t - Username: {};\n
               \t - Age: {};\n
               \t - comment: {};\n-----------------------------------------\n",
               self.username, self.age, self.comment)
    }
}
impl std::default::Default for Person {
    fn default() -> Self {
        Self {
            username: "John Doe".to_owned(),
            age: 34,
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
     pub fn from_json_vec(persons_vec_json: Vec<String>, timestamps: Vec<String>, target_file: String) -> Result<Self, serde_json::Error> {
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
            persons.insert(Metadata::new(timestamps.get(index).unwrap().clone() , index), person);
        }
        Ok(Self {
            persons: Some(persons),
            target_file,
        })
     }
     ///```markdown
     ///PersonLogger::new_empty()
     ///
     /// Creates an empty PersonLogger instance, used for state management basically
     /// ```
     pub fn new_empty(target_file: String) -> Self {
        Self {
            persons: None,
            target_file,
        }
     }
     /// ```markdown
     /// PersonLogger::append()
     ///
     /// Appends a given Vec<String> (of json interpretation) to PersonLogger
     /// if Option<HashMap<...>> is None, replaces it with persons_given_hash
     /// if Option<HashMap<...>> is Some(n), append given to that
     /// ```
     pub fn append(&mut self, persons_vec_json: Vec<String>, timestamps: Vec<String>) -> Result<(), serde_json::Error>{
        let persons_vec = self.json_to_vec_person(persons_vec_json)?;
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
                    file, "[{}]Username:{}-Age:{}-Comment:{}\n", 
                    metadata.timestamp,
                    person.username,
                    person.age,
                    person.comment) {
                    Ok(()) => {},
                    Err(e) => println!("{:#?}", e),
                };
            }

            self.persons = None;
            Ok(())} else {
                Err(std::io::ErrorKind::InvalidData.into())
            }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::collections::HashMap;
    use super::{PersonLogger, Metadata, Person};

    #[test]
    fn test_parse() {
        let json1 = json!(
        {
            "username": "Yessir",
            "age": 123,
            "comment": "y r u gey"
        }
        ).to_string();
        let json2 = json!(
        {
            "username": "Yessir",
            "age": 123,
            "comment": "y r u gey"
        }
        ).to_string();
        let json3 = json!(
        {
            "username": "Yessir",
            "age": 123,
            "comment": "y r u gey"
        }
        ).to_string();
        let vec_ts = vec!["00:00:00".to_owned(), "00:00:00".to_owned(), "00:00:00".to_owned()];
        let pl_hm = PersonLogger::from_json_vec(vec![json1, json2, json3], vec_ts, "idk".to_owned()).unwrap().persons.unwrap();
        let mut test_hm = HashMap::<Metadata, Person>::new();
        test_hm.insert(
            Metadata::new("00:00:00".to_owned(), 0),
            serde_json::from_str(
                json!(
                    {
                        "username": "Yessir",
                         "age": 123,
                         "comment": "y r u gey"
                      }
                ).to_string().as_str()
                ).unwrap()
        );
        test_hm.insert(
            Metadata::new("00:00:00".to_owned(), 1),
            serde_json::from_str(
                json!(
                    {
                        "username": "Yessir",
                         "age": 123,
                         "comment": "y r u gey"
                      }
                ).to_string().as_str()
                ).unwrap()
        );
        test_hm.insert(
            Metadata::new("00:00:00".to_owned(), 2),
            serde_json::from_str(
                json!(
                    {
                        "username": "Yessir",
                         "age": 123,
                         "comment": "y r u gey"
                      }
                ).to_string().as_str()
                ).unwrap()
        );
        assert_eq!(pl_hm, test_hm);
    }
}
