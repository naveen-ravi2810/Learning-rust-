use crate::operations::shape::Shape;
mod utils;

mod operations;

fn main() {
    println!("Welcome to the shaper");
    loop {
        println!();
        println!("1 -> Square\n2 -> Reactange\n3 -> Circle\n4 -> Triangle\n5 -> Exit");
        print!("Enter the choice: ");
        let _ = std::io::Write::flush(&mut std::io::stdout());
        let mut choice: String = String::new();
        let _ = match std::io::stdin().read_line(&mut choice) {
            Ok(a) => a,
            Err(_) => {
                println!("Enter the correct choice");
                continue;
            }
        };
        let choice: i32 = match choice.trim().parse() {
            Ok (n) => n,
            Err(_) => {
                println!("Not an interger \nKindly enter the choice");
                continue;
            }
        };
        let sh: Shape = 
        match choice {
            1 => operations::shape::Shape::square(),
            2 => operations::shape::Shape::rect(),
            3 => operations::shape::Shape::circle(),
            4 => operations::shape::Shape::triangle(),
            5 => break println!("Ok Bye"),
            _ => continue,
        };
        println!("The shape is {}\nThe area of the shape is {}\nThe perimeter of the shape is {}",sh.name, sh.area, sh.perimeter);
    }
}
