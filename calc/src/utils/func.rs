use colored::*;

pub fn get_input(text: &str) -> i32 {
    let mut val: String = String::new();
    print!("{}", text.bold().on_green());
    let _ = std::io::Write::flush(&mut std::io::stdout());
    std::io::stdin()
        .read_line(&mut val)
        .expect("Error in reading number");
    match val.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input for number and default value taken as 1 ");
            1
        }
    }
}