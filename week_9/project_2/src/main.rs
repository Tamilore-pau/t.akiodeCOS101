use std::io::Write;

fn main(){
    let mut file = std::fs::File::create("data.txt").expect("create failed");

    let first = ["Oluchi Mordi", "ACC10211111", "Accounting", "300"];
    let second = ["Adams Aliyu", "ECO10110101", "Economics", "100"];
    let third = ["Shania Bolade", "CSC10328828", "Computer", "200"];
    let fourth = ["Adekunle Gold","EEE11020202", "Electrical", "200"];
    let fifth = ["Bianca Edemoh", "MEE102001", "Mechanical","100"];
    file.write_all("Student Details\n"
        .as_bytes()).expect("write failed");
    file.write_all("Oluchi Mordi | ACC10211111 | Accounting | 300\n".as_bytes()).expect("write_failed");
    file.write_all("Adams Aliyu | ECO10110101 | Economics | 100\n" .as_bytes()).expect("write_failed");
    file.write_all("Shania Bolade | CSC10328828| Computer | 100\n".as_bytes()).expect("write_failed");
    file.write_all("Adekunle Gold | EEE11020202 | Electrical | 200\n".as_bytes()).expect("write_failed");
    file.write_all("Bianca Edemoh | MEE10202001 | Mechanical | 100\n".as_bytes()).expect("write_failed");


}
