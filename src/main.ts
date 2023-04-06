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
    username: string = "John Doe";
    age: number = 34;
    comment: string = "amen";
    constructor(
        username: string = "John Doe",
        age: number = 34,
        comment: string = "amen",
    ) {
        this.username = username;
        this.age = age;
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
    ): [string, number, string] {
        let { username, age, comment } = this;
        return [username, age, comment];
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
    id: number = PERSON_ARRAY.length;
    constructor(
        timestamp: string = new Date().toLocaleString(),
        id: number = PERSON_ARRAY.length,
    ) {
        this.id = id;
        this.timestamp = timestamp;
    }
    return_json(this: Metadata): string {
        return JSON.stringify(this);
    }
}
// ----------------------------------------------------------------------------------
// setting up event func
/**
 * Invokes necessary backend code for adding a person to the logger
 *
 * @remarks
 * This should be used as a last function (almost always)
 *
 * @param person_array: array of objects of class Person
 * @returns nothing (for now)
 */
var PERSON_ARRAY: Person[] = [];

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
        console.dir(person_json_string);
        console.dir(new Metadata().return_json());
        await invoke("accept_person_data", {
            personJsonString: person_json_string,
            metadata: new Metadata().return_json(),
        })
            .then((res) => {
                display_elem.innerText = <string>res;
                PERSON_ARRAY.push(person);
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
            console.dir(PERSON_ARRAY);
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
        const person: Person = new Person(
            (<HTMLInputElement>form_elem[0]).value,
            parseInt((<HTMLInputElement>form_elem[1]).value),
            (<HTMLInputElement>form_elem[2]).value,
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
