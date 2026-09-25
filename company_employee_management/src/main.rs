mod employees_management;

use std::{io, collections::HashMap};

fn main() {
    // K is department name, V is list of names of employees ;)
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please proceed with an action:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = input.trim();

        let option = employees_management::parse_options(input);
        match option {
            Ok(v) => {
                match v {
                    employees_management::Action::Exit => break,
                    employees_management::Action::Add((employee, department)) => {
                         departments
                            .entry(department.clone())
                            .or_default()
                            .push(employee.clone());

                        println!("{} added to {} department", employee, department);
                    },
                    employees_management::Action::Department(name) => {
                        let employees_sorted = 
                            employees_management::get_employees_by_department(&departments, name).unwrap(); // Goofy
                            println!("{employees_sorted:?}");
                    },
                    employees_management::Action::Company => {
                        let employees_sorted = 
                            employees_management::get_all_employees(&departments).unwrap(); // Goofy
                            println!("{employees_sorted:?}");
                    }

                }
            },
            Err(e) => {
                println!("Error: {e}");
                continue; // Start the loop again
            }
        }


    }
}
