use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Whats your age?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let age:u32 = input1.trim().parse().expect("Failed to read input");

    println!("Are you experienced? Type yes[X] or no[Z]");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let experienced = input2 .trim();
    let experience = experienced == "X";

    if experience{
        if age < 28 {
            println!("Your incentive is 1,300,000");
            
        }
        else if age <= 39 {
            println!("Your incentive is 1,480,000");

        }
        else if age >40 {
            println!("Your incentive is 1,560,000");
        }
      }
     else if experienced == "Z" {
        println!("Your incentive is 100,000");
     }
     else {
        println!("No incentive");
     }
}
