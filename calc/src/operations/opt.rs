use crate::utils::*;

pub fn add() -> i32 {
    println!("Addition Calculator");
    let n1: i32 = func::get_input("Enter number 1 ");
    let n2: i32 = func::get_input("Enter number 2 ");
    n1 + n2
}
pub fn subt() -> i32 {
    println!("Subtraction Calculator");
    let n1: i32 = func::get_input("Enter number 1 ");
    let n2: i32 = func::get_input("Enter number 2 ");
    n1 - n2
}
pub fn mult() -> i32 {
    println!("Multiplication Calculator");
    let n1: i32 = func::get_input("Enter number 1 ");
    let n2: i32 = func::get_input("Enter number 2 ");
    n1 * n2
}
pub fn div() -> f32 {
    println!("Multiplication Calculator");
    let n1: f32 = func::get_input("Enter number 1 ") as f32;
    let n2: f32 = func::get_input("Enter number 2 ") as f32;
    n1 / n2
}
