// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department 
// in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then, let the user retrieve 
// a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;

pub enum Action {
    Add((String, String)), // Employee name, Department to add
    Exit,                  // Leave the program
    Department(String),    // Get all employees by department (shorted alphabetically)
    Company,               // Get all employees in the company (shorted alphabetically)
}

pub fn parse_options(input: &str) -> Result<Action, String> {
    let parsed_values: Vec<&str> = input
                                    .split_whitespace()
                                    .collect();

    if parsed_values.len() == 0 {
        return Err(String::from("You should provide some parameters!"));
    }

    let option = parsed_values[0].to_lowercase(); // There will be always a 0 after the previous check
    if option == String::from("add") {
        if parsed_values[2].to_lowercase() != String::from("to") {
            return Err(String::from("Third parameters should be a `to`"));
        }

        return Ok(
            Action::Add(
                (
                    String::from(parsed_values[1]), 
                    String::from(parsed_values[3])
                )
            )
        );
    } else if option == String::from("exit") {
        return Ok(Action::Exit);
    } else if option == String::from("department") {
        return Ok(Action::Department(String::from(parsed_values[1])));
    } else if option == String::from("company") {
        return Ok(Action::Company);
    }

    Err(String::from("Option unrecognized"))
}

pub fn get_employees_by_department(departments: &HashMap<String, Vec<String>>, department_name: String) -> Option<Vec<String>> {
    let department = departments.get(&department_name)?;
    let mut sorted_names = department.clone();
    sorted_names.sort();
    Some(sorted_names)
}

pub fn get_all_employees(departments: &HashMap<String, Vec<String>>) -> Option<Vec<String>> {
    let mut all_employees: Vec<String> = Vec::new();

    if departments.is_empty() {
        return None;
    }

    for (_, v) in departments {
        all_employees.extend_from_slice(v);
    }

    all_employees.sort();

    Some(all_employees)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse_options() {
        let option = parse_options("Add Miguel to DevOps").unwrap();

        match option {
            Action::Add((employee, department)) => {
                assert_eq!(employee, String::from("Miguel"));
                assert_eq!(department, String::from("DevOps"));
            },
            _ => panic!("error asserting Add"),
        }
    }
    
}
