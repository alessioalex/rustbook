use std::collections::hash_map::Entry;
use std::collections::HashMap;

/*
Using a hash map and vectors,
create a text interface to allow a user to add employee names to a department in a company.
For example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department or all people
in the company by department, sorted alphabetically.
*/

#[derive(Debug)]
struct Company {
    departments: HashMap<String, Vec<String>>,
}

impl Company {
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    fn process(&mut self, text: &str) {
        let words: Vec<&str> = text.split_whitespace().collect();
        println!("{:?}", words);

        let action = words.get(0).unwrap();
        let employee = words.get(1).unwrap().to_string();
        let department = words.get(3).unwrap().to_string();

        assert_eq!(action.to_ascii_lowercase(), "add", "Missing 'add' command.");
        assert_eq!(
            employee.chars().count() > 1,
            true,
            "Employee name needs to have more than 1 character."
        );
        assert_eq!(
            department.chars().count() > 1,
            true,
            "Department name needs to have more than 1 character."
        );

        // let employees = self
        //     .departments
        //     .entry(department)
        //     .or_insert(vec![employee.clone()]);

        // if employees.len() >= 1 && !employees.contains(&employee) {
        //     employees.push(employee);
        // }
        match self.departments.entry(department) {
            Entry::Occupied(e) => {
                let employees = e.into_mut();

                if !employees.is_empty() && !employees.contains(&employee) {
                    employees.push(employee);
                }
            }
            Entry::Vacant(e) => {
                e.insert(vec![employee]);
            }
        }
    }

    fn get_employees_by_department(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }
}

fn main() {
    let mut company = Company::new();

    let text = "Add Sally to Engineering";
    company.process(text);
    let text = "Add Amir to Sales";
    company.process(text);
    let text = "Add Amir to Sales";
    company.process(text);
    let text = "Add Irine to Sales";
    company.process(text);

    println!("{:?}", company);
    println!("{:?}", company.get_employees_by_department("Sales"));
}
