struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    // Method to calculate the cost for a given quantity purchased
    fn cost_purchase(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Initialize the four laptop brands with their respective individual prices
    let laptops = vec![
        Laptop { brand: String::from("HP"), price: 650000 },
        Laptop { brand: String::from("IBM"), price: 755000 },
        Laptop { brand: String::from("Toshiba"), price: 550000 },
        Laptop { brand: String::from("Dell"), price: 850000 },
    ];

    let mut total_across_brands = 0;

    // Calculate the cost for purchasing 3 laptops from each brand
    for laptop in &laptops {
        let cost = laptop.cost_purchase(3);
        println!("3 {} laptops cost: ₦{}", laptop.brand, cost);
        total_across_brands += cost;
    }

    // Print the grand total cost
    println!("\nGrand total cost across all brands: ₦{}", total_across_brands);
}

