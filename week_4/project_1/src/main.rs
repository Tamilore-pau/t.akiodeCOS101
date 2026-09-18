// Rust program to find the roots of a quadratic equation
// given three values

use std::io;

fn main(){

   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

   println!("Input a");
   io::stdin().read_line(&mut input1).expect("Failed to print output");
   let a:f32 = input1.trim().parse().expect("Failed to print output");

   println!("Input b");
   io::stdin().read_line(&mut input2).expect("Failed to print output");
   let b:f32 = input2.trim().parse().expect("Failed to print output");

   println!("Input c");
   io::stdin().read_line(&mut input3).expect("Failed to print output");
   let c:f32 = input3.trim().parse().expect("Failed to print output");

   let d:f32 = b * b - 4.0 * a * c;

   if d > 0.0 {
    let root1 = (-b + d.sqrt())/2.0*a;
    let root2 = (-b - d.sqrt())/2.0*a;

    println!("There are two distincts roots and they are {} and {}", root1, root2);

   }
   else if d == 0.0 {
    let root = -b /2.0*a;
    println!("There is one real root and it is {}", root );

   }
   else if d<0.0 {
    println!("No roots");
   }
   
}