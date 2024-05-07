use colored::*;
mod operations;
mod utils;

fn main() {
    println!("{}","Welcome to the calculator".bold().bright_cyan());
    loop {
        let mut choice: String = String::new();
        println!("{}","Enter your choice\n1 -> Addition\n2 -> Subtraction\n3 -> Multiplication\n4 -> Division\n5 -> Exit".bright_yellow());
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Error in reading choice");
        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err( _ ) => i32::MAX
        };
        match choice {
            1 => println!("{}", operations::opt::add()),
            2 => println!("{}", operations::opt::subt()),
            3 => println!("{}", operations::opt::mult()),
            4 => println!("{:.2}", operations::opt::div()),
            5 => {
                println!("Task completed");
                break;
            }
            _ => {
                println!("Invalid option")
            }
        }
    }
}

/*
Gets the input from the the function with params as text input
*/


