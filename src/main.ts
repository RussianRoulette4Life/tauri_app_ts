import { invoke } from "@tauri-apps/api/tauri";

/**```markdown
 * A mirror class to the backend for ease of use
 *
 * @default has a default implementation, accessed by `let x = new Person()`
 *
 * @params username, age, comment
 * ```
 */
class Person {
  name: string = "John";
  middle_name: string = "Doe";
  last_name: string = "Unknowable";
  date_of_birth: string = "1.06.1970";
  comment: string = "amen";
  constructor(
    name: string = "John",
    middle_name: string = "Doe",
    last_name: string = "Unknowable",
    date_of_birth: string = "1.06.1970",
    comment: string = "amen",
  ) {
    this.name = name;
    this.middle_name = middle_name;
    this.last_name = last_name;
    this.date_of_birth = date_of_birth;
    this.comment = comment;
  }
  /** Converts a Person instance to a tuple of (Username, Age, Timestamp, Comment)
   *
   * @remarks
   * Basically useless for now
   *
   * @param this: takes in a Person instance
   */
  return_person_tuple(
    this: Person,
  ): [string, string, string, string, string] {
    let { name, middle_name, last_name, date_of_birth, comment } = this;
    return [name, middle_name, last_name, date_of_birth, comment];
  }
  return_json(this: Person): string {
    return JSON.stringify(this);
  }
}
/**```markdown
 * A mirror class to the backend for ease of use
 *
 * @default has a default implementation, accessed by `let x = new Metadata()`
 *
 * @params timestamp, id
 * ```
 */
class Metadata {
  timestamp: string = new Date().toLocaleString();
  id: number = ALL_PERSON_ARRAY.length;
  constructor(
    timestamp: string = new Date().toLocaleString(),
    id: number = ALL_PERSON_ARRAY.length,
  ) {
    this.id = id;
    this.timestamp = timestamp;
  }
  return_json(this: Metadata): string {
    return JSON.stringify(this);
  }
}
// ----------------------------------------------------------------------------------
// import {BaseDirectory, exists, writeFile} from "@tauri-apps/api/fs";
//
// await writeFile("app.txt", "huh", {dir: BaseDirectory.AppData}).then((res)=>{console.log(res)}, (err)=> {console.log(err)});
var ALL_PERSON_ARRAY: Person[] = [];
var NEW_PERSON_ARRAY: Person[] = [];
/**
 * Invokes necessary backend code for adding a person (or multiple persons) to the logger on the backend
 *
 * the PERSON_ARRAY is appended a Person object after this function
 * @remarks
 * There will be no PersonLogger on the frontend! Too complicated for what needs to be done!
 *
 * @param display_elem: is there to display the answer of the backend
 * @returns nothing (no promises?!)
 */
const send_person_data = async function(
  person_array: Person[],
  display_elem: HTMLElement,
) {
  // for each Person instance call the backend
  for (const person of person_array) {
    const person_json_string = person.return_json();
    await invoke("accept_person_data", {
      personJsonString: person_json_string,
      metadata: new Metadata().return_json(),
    })
      .then((res) => {
        display_elem.innerText = <string>res;
        ALL_PERSON_ARRAY.push(person);
      }, (e) => {
        display_elem.innerText = e;
      });
  }
};
/**
 * Invokes necessary backend code for flushing the logger to disk
 *
 * the logger object is then reset to a default state
 * @remarks
 * Please note that, again there will be no Logger class on the frontend, there will only be a PERSON_ARRAY
 *
 * @param display_elem: is there to display the answer of the backend
 * @returns nothing (no promises?!)
 */
const flush_logger = async function(display_elem: HTMLElement) {
  await invoke("flush_logger")
    .then((res) => {
      display_elem.innerText = <string>res;
    }, (e) => {
      display_elem.innerText = e;
    });
};
/**
 * Returns a JSON string of all Person instances of the PersonLogger backend struct
 * @param: none lol
 * @remarks: will probably be reworked as a way to display all people as a way to render a student table
 * @returns: nothing
 * Will need a display_elem sooner
 */
const json = async function() {
  await invoke("json")
    .then(() => {
      console.dir(ALL_PERSON_ARRAY);
    });
};
window.addEventListener("DOMContentLoaded", () => {
  // setting up vars
  let form_elem: HTMLFormElement;
  let header_elem: HTMLElement;

  form_elem = document.querySelector("#form_elem")!;
  header_elem = document.querySelector("#header")!;
  // event listener on main form
  form_elem?.addEventListener("submit", (event: Event) => {
    // prevent the default reload action, build the Person from form selectors
    event.preventDefault();
    form_elem.getElementsByTagName;
    // this is ugly as hell but what has to be done will be done
    let name_element = <HTMLInputElement>form_elem[0];
    let middle_name_element = <HTMLInputElement>form_elem[1];
    let last_name_element = <HTMLInputElement>form_elem[2];
    let date_of_birth_element = <HTMLInputElement>form_elem[3];
    let comment_element = <HTMLInputElement>form_elem[4];

    const person: Person = new Person(
      name_element.value,
      middle_name_element.value,
      last_name_element.value,
      date_of_birth_element.value,
      comment_element.value,
    );
    // checking what button was pressed
    if ((<SubmitEvent>event).submitter!.id === "add_person") {
      // send person to backend
      send_person_data([person], header_elem);
    } else if ((<SubmitEvent>event).submitter!.id === "get_json") {
      json();
    } else if ((<SubmitEvent>event).submitter!.id === "write_to_disk") {
      // flush the logger to disk
      flush_logger(header_elem);
    }
  });
});
