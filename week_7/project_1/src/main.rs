 use std::io;
use std::io::Write;
use std::f64::consts::PI;
fn main(){
    println!("MTH 101 Shape Calculator");
    println!("Select shape:");
    println!("1. Trapezium");
    println!("2. Rhombus");
    println!("3. Parallelogram");
    println!("4. Cube");
    println!("5. Cylinder");
    println!("6. Exit");

    //ask for their choice sir
    println!("\nEnter your choice (1,2,3,4,5,6):");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid input");
    let choice :u32 = match choice.trim().parse() {
    Ok(num) => num,
    Err(_) => {
    println!("Your choice is {}", choice);
    return;
}
};

    //match choice to function sir
    match choice {
        1 => {
            println!("\n---Trapezium Area---");
            let height = read_input("Enter height: ");
            let base1 = read_input("Enter base1: ");
            let base2 = read_input("Enter base2: ");
            let area = calculate_trapezium_area(height, base1, base2);
            println!("Your area of trapezium is: {:.2}", area);
        }
        2 => {
            println!("\n---Rhombus Area---");
            let diagonal1 = read_input("Enter diagonal1: ");
            let diagonal2 = read_input("Enter diagonal2: ");
            let area = calculate_rhombus_area(diagonal1, diagonal2);
            println!("Your area of rhombus is {:.2}", area);
        }
        3 => {
            println!("\n---Parallelogram Area---");
            let base = read_input("Enter base: ");
            let altitude = read_input("Enter altitude: ");
            let area = calculate_parallelogram_area(base, altitude);
            println!("Your area of Parallelogram is {:2}", area);
        }
        4 => {
            println!("\n---Cube Surface Area---");
            let side = read_input("Enter side length: ");
            let surface_area = calculate_cube_surface_area(side);
            println!("Your surface area of cube is {:.2}", surface_area);
        }
        5 => {
            println!("\n--- Cylinder Volume ---");
            let radius = read_input("Enter radius: ");
            let height = read_input("Enter height: ");
            let volume = calculate_cylinder_volume(radius, height);
            println!("The volume of the cylinder is: {:.2}", volume);
        }
        6 => println!("Exiting program. Goodbye!"),
        _ => println!("Invalid choice. Please run the program again and select 1-6."),
    }
}
fn read_input(prompt: &str) -> f64 {
loop {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match input.trim().parse::<f64>() {
    Ok(num) if num >= 0.0=> return num,
    _ => println!("Please enter a valid number"),
}
}//end of loop
}//end of fn

    // 1. Function for Trapezium Area
fn calculate_trapezium_area(height: f64, base1: f64, base2: f64) -> f64 {
    (height / 2.0) * (base1 + base2)
}

// 2. Function for Rhombus Area
fn calculate_rhombus_area(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

// 3. Function for Parallelogram Area
fn calculate_parallelogram_area(base: f64, altitude: f64) -> f64 {
    base * altitude
}

// 4. Function for Cube Surface Area
fn calculate_cube_surface_area(side: f64) -> f64 {
    6.0 * side * side
}

// 5. Function for Cylinder Volume
fn calculate_cylinder_volume(radius: f64, height: f64) -> f64 {
    PI * radius * radius * height
}
    

