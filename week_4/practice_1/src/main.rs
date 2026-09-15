// Rust program to output name and age

use std::io;

fn main() {
    println!("Student Information Management System!");

    //why does ronaldo clean his room,cause he is not messi
    println!("\nPlease Enter your name,");
    let mut name = String::new();
        io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");
    println!("Your name is: {}", name);

    //if u liked the previous joke,wear black to the next class sir
    println!("\nEnter your age.");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);
}    
