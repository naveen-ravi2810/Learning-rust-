pub fn get_input(s: &str) -> u32 {
    let mut a: String = String::new();
    print!("{}",s);
    let _ = std::io::Write::flush(&mut std::io::stdout());
    std::io::stdin()
            .read_line(&mut a)
            .expect("Error in reading input");
    let a: u32 = match a.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Cannot take {} as input default 1 will be assigned", a);
            1
        }
    };
    a
}