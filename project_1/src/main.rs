use std::io;
//display menu
fn main(){
    println!("====RESTAURANT MENU====");
    println!("P - Poundo Yam / Edinkaiko Soup - N3200");
    println!("F - Fried rice & Chicken - N3000");
    println!("A - Amala & Ewedu soup - N2500");
    println!("E - Eba & Egusi Soup - N2000");
    println!("W - White Rice & Stew - N2500");

    //ask for food type
    println!("Enter food type your majesty (P, F, A, E, W):");
    let mut food = String::new();
    io::stdin().read_line(&mut food).unwrap();
    let food = food.trim().to_uppercase();

    //ask for quantity
    println!("Enter quantity your majesty");
    let mut quantity = String::new();
    io::stdin().read_line(&mut quantity).unwrap();
    let quantity :i32 = quantity.trim().parse().unwrap();

    //find the price
    let price :i32;
    match food.as_str() {
        "P" => price = 3200,
        "F" => price = 3000,
        "A" => price = 2500,
        "E" => price = 2000,
        "W" => price = 2500,
         _=> {
            println!("Invalid food type.");
            return;
        }
    }
    //calculate total charge sir
    let total = price * quantity;
    println!("Total before discount: N{}", total);

    //calculate discount sir
    if total > 10000 {
        let discount = total as f64 * 0.05;
        let final_price = total as f64 - discount;

        println!("Discount: N{}", discount);
        println!("Final amount: N{}", final_price);
    } else {
        println!("No discount.");
        println!("Final amount: N{}",total);
    }
    
}

