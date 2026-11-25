use std::io::{self, Read};

fn main(){
    let mut role = String::new();
    println!("Enter your role\nAdministrator(a)\nProject Manager(p)\nEmployee(e)\nCustomer(c)\nVendor(v):");
    io::stdin().read_line(&mut role).expect("Could not read value!");
    let role = role.trim();

    let mut file_name = "";
    if role == "a" {
        file_name = "globacom_db.sql";
    }
    else if role == "p" {
        file_name = "project_table.sql";
    }
    else if role == "e" {
        file_name = "staff_table.sql";
    }
    else if role == "c" {
        file_name = "customer_table.sql";
    }
    else if role == "v" {
        file_name = "dataplan_table.sql";
    }
    else {
        println!("Invalid Input!");
    }
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}