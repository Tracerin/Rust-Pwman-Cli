use std::io;
use std::process::Command;
use std::process::Output;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

struct Savedapp {
    app_name: String,
    username: String,
    email: String,
    png_name: String,
    num_chars: i32,
    // implement valid chars later
}
fn convert_png_to_string(rel_path: String, pin: &u64, pw_length: i32) -> String  {
    let mut strings_com = Command::new("strings");
    strings_com.arg(rel_path);
    let raw_output: Output = strings_com.output().expect("failed to run strings command");
    let raw_string: String = String::from_utf8(raw_output.stdout).unwrap();
    let mut final_pw: String = String::new();
    for _n in 0..pw_length {
        final_pw += "a";
    } 
    //let mut rng = ChaCha8Rng::seed_from_u64(*pin);
    return raw_string;
}

fn display_menu() {
    println!("1. Add new password \n2. View Saved Passwords \n3. Exit");
}

fn confirm_pin(user_confirm: &str) {

}

fn display_saved_pws(mut saved_app_vec: Vec<Savedapp>) {
    for Savedapp in &mut saved_app_vec {
        println!("App Name: {}\n    Username: {}\n    Email: {}\n", Savedapp.app_name, Savedapp.username, Savedapp.email);
    }
}

fn display_specific_pw(saved_app: Savedapp, pin: &u64) {
    let path_header: String = "/src/imgsrc/".to_string();
    let rel_path = path_header + &saved_app.png_name;
    let pw = convert_png_to_string(rel_path, pin, saved_app.num_chars);
    println!("App Name: {}\n    Username: {}\n    Email: {}\n    Password: {}", saved_app.app_name, saved_app.username, saved_app.email, pw);
}

fn main() {
    // current directory: home/repos/cli_pwman_rust
    // example path for pngs: src/imgsrc/<example.png>
    // let raw_output = convert_png_to_string(relpath1); -- working func
    // let raw_string: String = String::from_utf8(raw_output.stdout).unwrap(); // convert from output to string
    //let valid_chars: [char; 3] = ['a', 'b', 'c']; 
    let example_app = Savedapp {
        app_name: String::from("Chrome"),
        username: String::from("Trace"),
        email: String::from("tracerindal@gmail.com"),
        png_name: String::from("bezospog.png"),
        num_chars: 10,
    };
   
    
    let mut saved_app_vec: Vec<Savedapp>;

    // parse json here - create SavedApp struct for each application and append to saved_app_vec
    let mut input: String = String::new();
    println!("Please enter your pin:");
    io::stdin().read_line(&mut input).expect("Failed to read pin"); 
    let mut pin: u64 = input.trim().parse().expect("Invalid input"); // read_line() reads line of input including \n, so need trim to get rid of newline before converting to u32
    // save var pin for seed for rng
    println!("Your pin: {}, Type Y/n to confirm/deny", pin);
    
    let mut confirm: String = String::new();
    io::stdin().read_line(&mut confirm).expect("Please input valid char"); 
    
    match confirm.as_str() {
        "yes"|"Yes"|"y"|"Y"=>{
            println!("Pin: {}, confirmed", pin);
        },
        "no"|"No"|"n"|"N"=> {
            println!("Incorrect pin, please input new pin:");
            io::stdin().read_line(&mut input).expect("Failed to read pin"); 
            pin = input.trim().parse().expect("Invalid input"); // read_line() reads line of input including \n, so need trim to get rid of newline before converting to u32
            // save var pin for seed for rng
            println!("Your pin: {}, Type Y/n to confirm/deny", pin);
        },
        _ => println!("Please put in Y/n, confirming that {} is your pin", pin),
    }
    
    display_menu();

}
