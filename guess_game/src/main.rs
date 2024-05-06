use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Welcome to guess game");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret_number is {secret_number}");
    println!("Enter the number");
    // let mut guess: String = String::new();
    // io::stdin().read_line(&mut guess).expect("Error in reading value");
    // let guess: u32= guess.trim().parse().expect("Error in parsing to int"); 
    // match guess.cmp(&secret_number) {
    //     Ordering::Equal => println!("The both numbers are equal"),
    //     Ordering::Greater => println!("The number you entered in greated than the secret number"),
    //     Ordering::Less => println!("The number you entered in lesser than the secret number"),
    // }
    loop {
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Error in reading value");
        let guess: u32= guess.trim().parse().expect("Error in parsing to int"); 
        match guess.cmp(&secret_number) {
            Ordering::Equal => break,
            Ordering::Greater => println!("The number you entered in greated than the secret number"),
            Ordering::Less => println!("The number you entered in lesser than the secret number"),
        }
    }
    println!("YOu have won");
}
