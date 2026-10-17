fn main() {

    let candidates = vec![
        ("Chidi", 5),
        ("Fatima", 12),
        ("Amina", 8),
        ("Olumide", 15),
        ("Zainab", 3),
    ];

    // look for the candidate with the highest experience sir
    let mut highest_candidate = candidates[0];
    for candidate in &candidates {
        // Compare the years of experience sir
        if candidate.1 > highest_candidate.1 {
            highest_candidate = *candidate;
        }
    }

    // Output the results sir
    println!("--- EY Nigeria Recruitment Results ---");
    println!("The selected candidate with the highest experience is:");
    println!("Name: {}", highest_candidate.0);
    println!("Experience: {} years", highest_candidate.1);
}

