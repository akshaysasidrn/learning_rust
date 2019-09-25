// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company. For example, “Add Sally to
// Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
//  people in a department or all people in the company by department, sorted
// alphabetically.

use std::io;
use std::collections::HashMap;

struct Employee {
    name: String,
    dept: String,
}

impl Employee {
    fn new() -> Employee {
        Employee { name: String::new(), dept: String::new() }
    }
}

enum Command {
    Add(Employee),
    Retrieve(String),
    Exit,
    Error(&'static str),
}

fn add_employee(employee: Employee, departments: &mut HashMap<String, Vec<String>>) {
    let dept = &employee.dept;

    if departments.contains_key(dept) {
        let names = departments.get_mut(dept).unwrap();
        names.push(employee.name);
        names.sort();
    } else {
        departments.insert(employee.dept, vec![employee.name]);
    }
}

fn retrieve_all(departments: &HashMap<String, Vec<String>>) {
    for key in departments.keys() {
        retrieve_dept(key.to_string(), departments);
    }
}

fn retrieve_dept(department: String, departments: &HashMap<String, Vec<String>>) {
    if let Some(names) = departments.get(&department) {
        println!("{}", &department);
        for name in names {
            println!("  {}", name);
        }
    } else {
        println!("No {} department", department);
    }
}

fn ask_for_command() -> String {
    println!("Please enter your command:");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn process_command(command: String) -> Command {
    let mut words = command.split_whitespace();

    match words.next() {
        Some("Add") => {
            if let Some(employee_name) = words.next() {
                let mut employee = Employee::new();
                let mut before_to = true;

                employee.name.push_str(employee_name);

                loop {
                    if let Some(word) = words.next() {
                        if word == "to" {
                            before_to = false;
                            continue;
                        }

                        if before_to {
                            // concatenate to employee name
                            employee.name.push_str(&format!(" {}", word));
                        } else {
                            if employee.dept.len() != 0 {
                                // concatenate to department
                                employee.dept.push_str(" ");
                            }
                            employee.dept.push_str(word);
                        }
                    } else {
                        break;
                    }
                }

                if employee.dept == "" {
                    Command::Error("Department not provided")
                } else {
                    Command::Add(employee)
                }
            } else {
                Command::Error("Employee name not provided")
            }
        },
        Some("Retrieve") => {
            if let Some(word) = words.next() {
                if word == "all" {
                    Command::Retrieve(word.to_string())
                } else {
                    Command::Retrieve(command[9..].trim().to_string())
                }
            } else {
                Command::Error("Department not provided")
            }
        },
        Some("exit") => Command::Exit,
        _ => Command::Error("Invalid command"),
    }
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter 'Add <name> to <department>' to add person");
        println!("Enter 'Retrieve <department>' or 'Retrieve all'");
        println!("Enter 'exit' to exit");

        let input = ask_for_command();

        match process_command(input) {
            Command::Add(employee) => add_employee(employee, &mut departments),
            Command::Retrieve(dept) => {
                if let "all" = &dept[..] {
                    retrieve_all(&mut departments);
                } else {
                    retrieve_dept(dept, &mut departments);
                }
            },
            Command::Exit => break,
            Command::Error(e) => println!("Error parsing command: {}", e),
        }

        println!("\n");
    }
}
