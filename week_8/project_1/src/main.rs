use std::io;

fn main(){
    let _office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let _academic = vec!["Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let _lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let _teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    let _public_servant = vec!["APS 1-2", "APS 3-4", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let mut v : Vec<&str> = Vec::new();

    let mut experience = String::new();
    println!("How many years of experience do you have?");
    io::stdin().read_line(&mut experience).unwrap();
    let experience :usize = experience.trim().parse().expect("Failed to read ");

//     loop {

//     if experience >= 1 {
//         println!("you have {} years of experience ", experience);
//     } else if experience >= 3 {
//         println!("you have {} years of experience ", experience);
//     } else if experience >= 5 {
//         println!("you have {} years of experience ", experience);
//     } else if experience >= 8 {
//         println!("you have {} years of experience ", experience);
//     } else if experience >= 10 {
//         println!("you have {} years of experience ", experience);
//     } else if experience >= 14 {
//         println!("you have {} years of experience ", experience);
//     }
    
//    break; 
// }
   
    
    let mut occupation = String::new();
    println!("Whats is your occupation?: 1.Office admin 2.Academic 3.Lawyer 4.Teacher");
    io::stdin().read_line(&mut occupation).unwrap();
    let occupation = occupation.trim();

    let mut position = String::new();
    println!("Enter your position:");
    io::stdin().read_line(&mut position).unwrap();
    let position = position.trim();

    
    match occupation {
        "Office Admin" => {
            v.push(&occupation);
            if position == "Intern" && (experience > 0 && experience <= 2) {
                v.push(&position);
                v.push("APS 1-2");
            }
            else if position =="Administrator"   && (experience >=3 && experience <=5) {
                v.push(&position);
                v.push("APS 3-5");
            }
            else if position == "Senior Administration" && (experience >=5 && experience <=8) {
                v.push(&position);
                v.push("APS 5-8");
            }
            else if position == "Office Manager" && (experience >=8 && experience <=10){    
                v.push(&position);
                v.push("EL1 8-10");
            }
            else if position == "Director" && (experience >=10 && experience <=13){
                v.push(&position);
                v.push("EL2 10-13");
            }
            else if position == "CEO" && (experience <=14){ 
                v.push(&position);
                v.push("SES");
            }
        },
        "Academic" => {
            v.push(&occupation);
            if position =="Research Assistant" && (experience >=3 && experience <=5) {
                v.push(&position);
                v.push("APS 3-5");
            }
            else if position == "PhD Candidate" && (experience >=5 && experience <=8) {
                v.push(&position);
                v.push("APS 5-8");
            }
            else if position == "Post-Doc Researcher" && (experience >=8 && experience <=10){    
                v.push(&position);
                v.push("EL1 8-10");
            }
            else if position == "Senior Lecturer" && (experience >=10 && experience <=13){
                v.push(&position);
                v.push("EL2 10-13");
            }
            else if position == "Dean" && (experience >=14){ 
                v.push(&position);
                v.push("SES");
            }
        },
        "Lawyer" => {
             v.push(&occupation);
            if position == "Paralegal" && (experience > 0 && experience <= 2) {
                v.push(&position);
                v.push("APS 1-2");
            }
            else if position =="Junior Associate"   && (experience >=3 && experience <=5) {
                v.push(&position);
                v.push("APS 3-5");
            }
            else if position == "Associate" && (experience >=5 && experience <=8) {
                v.push(&position);
                v.push("APS 5-8");
            }
            else if position == "Senior Associate 1-2" && (experience >=8 && experience <=10){    
                v.push(&position);
                v.push("EL1 8-10");
            }
            else if position == "Senior Associate 3-4" && (experience >=10 && experience <=13){
                v.push(&position);
                v.push("EL2 10-13");
            }
            else if position == "Partner" && (experience >=14){ 
                v.push(&position);
                v.push("SES");
            }
        },
        "Teacher" => {
            v.push(&occupation);
            if position == "Placement" && (experience > 0 && experience <= 2) {
                v.push(&position);
                v.push("APS 1-2");
            }
            else if position =="Classroom Teacher"   && (experience >=3 && experience <=5) {
                v.push(&position);
                v.push("APS 3-5");
            }
            else if position == "Senior Teacher" && (experience >=5 && experience <=8) {
                v.push(&position);
                v.push("APS 5-8");
            }
            else if position == "Leading Teacher" && (experience >=8 && experience <=10){    
                v.push(&position);
                v.push("EL1 8-10");
            }
            else if position == "Deputy Principal" && (experience >=10 && experience <=13){
                v.push(&position);
                v.push("EL2 10-13");
            }
            else if position == "Principal" && (experience >=14){ 
                v.push(&position);
                v.push("SES");
            }
        },
        _ => println!("Invalid Qualifications"),
    }

    if v.is_empty() {
        println!("Invalid Qualifications");
    }
    else {
        println!("Your information:");
        for t in v {
            println!("{}", t);
        }
    }
}













