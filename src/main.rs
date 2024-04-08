use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter your pin:");
    io::stdin().read_line(&mut input).expect("Failed to read pin");
    let pin: i32 = input.trim().parse().expect("Invalid input");
    println!("Your pin: {}", pin);
}
