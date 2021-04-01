extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

const MAX_DAYS: usize = 40;
const NUM_OF_PEOPLE: usize = 15;

#[derive(Debug)]
struct Person {
    days: [Option<Mark>; MAX_DAYS],
    priority: Option<u32>,
    room: u32,
}

#[derive(Copy, Clone, Debug)]
enum Mark {
    Mark,
}

fn main() {
    let file = fs::read_to_string("mad.csv").expect("cannot read file");
    let persons = setup(&file);
    run(persons);
}

fn run(mut persons: Vec<Person>) {
    let mut mad_doodle = Vec::new();
    for day in 0..MAX_DAYS {
        // Find the person with the max priority.
        let mut max_person = None;
        for person in persons.iter_mut() {
            match (&max_person, person.priority, person.days[day]) {
                // If there is no max priority person yet, set the current person to be the max
                // priority person. The person needs to have a mark on the current day to be set as max_person.
                (None, _, Some(Mark::Mark)) => {
                    max_person = Some(person)
                }
                // If there already is a max priority person. Check if the current person is less than
                // the max priority person. 
                (Some(max_p), Some(priority), Some(Mark::Mark)) => {
                    if let Some(max_priority) = max_p.priority {
                        if priority > max_priority {
                            max_person = Some(person)
                        }
                    }
                },
                // In all other cases, do nothing.
                _ => (),
            };
        }

        for person in persons.iter_mut() {
            match person.priority {
                Some(mut p) => p += 1,
                None    => (),
            }
        }

        match max_person {
            Some(mut max_person) => {
                max_person.priority = Some(0);
                mad_doodle.push(Some(max_person.room));
            },
            None => mad_doodle.push(None),
        }
    }
}

fn setup(file: &str) -> Vec<Person> {
    let parser = CSVParser::parse(Rule::file, file)
        .expect("unsuccessfull parse")
        .next().unwrap();
    let mut parser = parser.into_inner();

    let mut persons = Vec::with_capacity(NUM_OF_PEOPLE);

    // Fill the persons vector with each person, consiting of their room number and the other
    // fields initalized with None.
    let mut rooms = parser.next().unwrap().into_inner();
    rooms.next();
    for room in rooms {
        let room_num = room.as_str().parse::<u32>().unwrap();
        let person = Person { days: [None; MAX_DAYS], 
                       priority: None,
                       room: room_num };
        persons.push(person);
    }
      
    // Fill in the days the persons have marked.
    for row in parser {
        let mut row = row.into_inner();
          
        // Getting the day of that row
        let day = match row.next() {
            Some(d) => d,
            None    => continue,
        };
        let mut day = day
            .into_inner().next().unwrap().into_inner();
        day.next();
        let day = day
            .next().unwrap().as_str().parse::<usize>().unwrap();

        // Setting a mark if for that particular day they have marked
        let mut room_num: usize = 0;
        for col in row {
            // Get the inner fields of the field rule
            let col = col.into_inner().next().unwrap();
            match col.as_rule() {
                Rule::x => {
                    persons[room_num].days[day - 1] = Some(Mark::Mark);
                }
                _ => (),
            }
            room_num += 1;
        }
    }
    persons
}
