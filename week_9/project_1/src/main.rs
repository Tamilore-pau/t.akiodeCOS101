use std::io::Write;

fn main(){
    let mut file = std::fs::File::create("Nigerian Breweries").expect("wrong input");
    let mut contents = String::new();
    file.write_all("Welcome, This is Nigerian Breweries plc\n"
        .as_bytes()).unwrap();
    println!("{}", contents);

    let contents = ["Lager", "Stout", "Non_Alcoholic"];
    let lager = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = ["Legend", "Turbo King", "Williams"];
    let non_alcoholic = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];
    
    file.write_all("contents: Lager, Stout, Non_Alcoholic\n".as_bytes()).expect("Failed to store");
    file.write_all("lager: 33 Export, Desperados, Goldberg, Gulder, Heineken, Star\n".as_bytes()).expect("Failed to store");
    file.write_all("stout: Legend, Turbo King, Williams\n".as_bytes()).expect("Failed to store");
    file.write_all("Non-Alcoholic: Maltina, Amstel Malta, Malta Gold, Fayrouz\n".as_bytes()).expect("Failed to store");
    println!("==File stored==");
    

}

