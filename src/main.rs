use std::io;
use std::fs;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct Savedapp {
    app_name: String,
    username: String,
    email: String,
    png_name: String,
    num_chars: i32,
}


fn read_file_as_bytes(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn bytes_as_ascii_string(all_bytes: &Vec<u8>) -> String {
    let mut filter_vec: Vec<u8> = Vec::new();
    for &i in all_bytes {
        if (i >= 33) && (i <= 126) { // is printable character, avoids space
            filter_vec.push(i);
        }
    }

    let valid_ascii_string = String::from_utf8(filter_vec).unwrap();
    return valid_ascii_string;
}

fn convert_png_to_string(rel_path: &String, pin: &u64, pw_length: &i32) -> String  {

    let raw_string = bytes_as_ascii_string(&read_file_as_bytes(&rel_path).unwrap());
    let mut final_pw: String = String::new();

    let mut rng = ChaCha8Rng::seed_from_u64(*pin);
    let rng_val = rng.gen_range(1..10);

    let mut i = 0;
    for (index, character) in raw_string.chars().enumerate() {
        // Check if it's the 8th character (index is 0-based)
        if (index + 1) % rng_val == 0 && character != '\n' && character != ' ' {
            final_pw.push(character);
            i += 1;
        }
        if &i == pw_length {
            break;
        }
    }
    return final_pw;
}

fn delete_pw(mut saved_app_vec: Vec<Savedapp>, pin: &u64) {
    for savedapp in &saved_app_vec {
        println!("App Name: {}\n    Username: {}\n    Email: {}\n", savedapp.app_name, savedapp.username, savedapp.email);
    }
    println!("Please type <App Name> to remove saved password:");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read App Name");
    input = input.trim().to_string();

    if input.as_str() == "exit" {
        print!("{esc}c", esc = 27 as char);
        println!("Goodbye!");
        std::process::exit(0);
    }
    else if input.as_str() == "menu" {
        print!("{esc}c", esc = 27 as char);
        println!("Here you go!");
        display_menu(saved_app_vec, pin);
    }
    else {
        let mut valid_app: bool = false;
        let temp_app_name = &input;
        let mut i = 0;
        while i < saved_app_vec.len() {
            if valid_app == true {
                break;
            }
            if saved_app_vec[i].app_name == input.as_ref(){
                saved_app_vec.remove(i);
                valid_app = true;
            } 
            else {
                i += 1;
            }
        }

        if valid_app == false {
            print!("{esc}c", esc = 27 as char);
            println!("Invalid app name, taking you back to main menu");
            display_menu(saved_app_vec, pin);
        } 
        else {
            let json_string = serde_json::to_string_pretty(&saved_app_vec)
            .expect("Failed to serialize Savedapp vector to JSON");
    
            fs::write("src/saved_apps.json", json_string).expect("Failed to write JSON to file"); // rewrites whole file, not sure if more efficient way
       
            // then go back to menu
            print!("{esc}c", esc = 27 as char);
            println!("Successfully deleted app: {}", temp_app_name);
            display_menu(saved_app_vec, pin);
        }
    }

}

fn display_menu(saved_app_vec: Vec<Savedapp>, pin: &u64) {
    println!("Please select from the following: \n1. Add new password \n2. View Saved Passwords \n3. Remove a password \n4. Exit");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read pin");
    match input.trim().as_ref() {
        "1" => {
            add_new_password(saved_app_vec, pin);
        },
        "2" => {
            print!("{esc}c", esc = 27 as char);
            display_saved_pws(saved_app_vec, pin);
        },
        "3" => {
            print!("{esc}c", esc = 27 as char);
            delete_pw(saved_app_vec, pin);
        },
        "4" => {
            print!("{esc}c", esc = 27 as char);
            println!("Goodbye!");
            std::process::exit(0);
        },
        _ => {
            print!("{esc}c", esc = 27 as char);
            println!("Invalid option, please select again:"); 
            display_menu(saved_app_vec, pin);
        }, 
    }
}


fn display_saved_pws(saved_app_vec: Vec<Savedapp>, pin: &u64) {
    for savedapp in &saved_app_vec {
        println!("App Name: {}\n    Username: {}\n    Email: {}\n", savedapp.app_name, savedapp.username, savedapp.email);
    }
    println!("Please type <App Name> to view a specific password\nAlternatively, please type \"menu\" or \"quit\" to either go back to menu or to quit.");

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read App Name");
    input = input.trim().to_string();
    if input.as_str() == "exit" {
        print!("{esc}c", esc = 27 as char);
        println!("Goodbye!");
        std::process::exit(0);
    }
    else if input.as_str() == "menu" {
        print!("{esc}c", esc = 27 as char);
        println!("Here you go!");
        display_menu(saved_app_vec, pin);
    }
    else {
        let mut valid_app: bool = false;
        for savedapp in &saved_app_vec {
            if input.as_ref() == savedapp.app_name {
                valid_app = true;
                display_specific_pw(&savedapp, pin);
            }
            if valid_app == true {
                break;
            }
        }
        if valid_app == false {
            print!("{esc}c", esc = 27 as char);
            println!("Invalid app name, taking you back to main menu");
            display_menu(saved_app_vec, pin);
        }
    }
}

fn display_specific_pw(saved_app: &Savedapp, pin: &u64) {
    let path_header: String = "src/imgsrc/".to_string();
    let rel_path = path_header + &saved_app.png_name;
    let pw = convert_png_to_string(&rel_path, &pin, &saved_app.num_chars);
    print!("{esc}c", esc = 27 as char);
    println!("Here you go!");
    println!("App Name: {}\n    Username: {}\n    Email: {}\n    Password: {}", saved_app.app_name, saved_app.username, saved_app.email, pw);
}

fn new_pw_confirmation_match() -> bool {
    println!("Please type Y/n to confirm new addition");
    let mut confirmation = String::new();
    io::stdin().read_line(&mut confirmation).expect("Failed to read confirmation");

    match confirmation.trim().as_ref() {
        "Y"|"y"|"Yes"|"yes" => {
            return true;
        },
        "N"|"n"|"No"|"no" => {
            return false;
        },
        _ => {
            let temp_bool = new_pw_confirmation_match();
            return temp_bool;
        },
    }
}

fn add_new_password (mut saved_app_vec: Vec<Savedapp>, pin: &u64) {
    let mut input_app_name = String::new();
    println!("Please enter app name: ");
    io::stdin().read_line(&mut input_app_name).expect("Failed to read app name");
    input_app_name = input_app_name.trim().to_string();

    let mut input_username = String::new();
    println!("Please enter username or leave blank: ");
    io::stdin().read_line(&mut input_username).expect("Failed to read username");
    input_username = input_username.trim().to_string();

    let mut input_email = String::new();
    println!("Please enter email or leave blank: ");
    io::stdin().read_line(&mut input_email).expect("Failed to read email");
    input_email = input_email.trim().to_string();

    let mut input_png_name = String::new();
    println!("Please enter png name and confirm png is in imgsrc folder: ");
    io::stdin().read_line(&mut input_png_name).expect("Failed to read png name");
    input_png_name = input_png_name.trim().to_string();

    let mut input_num_chars = String::new();
    println!("Please enter number of characters you wish to have: ");
    io::stdin().read_line(&mut input_num_chars).expect("Failed to read number of characters");
    let int_num_chars: i32 = input_num_chars.trim().parse().expect("Invalid input");
   

    let new_app = Savedapp {
        app_name: input_app_name,
        username: input_username,
        email: input_email,
        png_name: input_png_name,
        num_chars: int_num_chars,
    };

    println!("App Name: {}\n    Username: {}\n    Email: {}\n    Png Name: {}\n    Number of Characters: {}\n", new_app.app_name, new_app.username, new_app.email, new_app.png_name, new_app.num_chars);
    let confirm_new_app: bool = new_pw_confirmation_match();

    if confirm_new_app == true {
        // add to saved app vec and write to json
        saved_app_vec.push(new_app);
    
        let json_string = serde_json::to_string_pretty(&saved_app_vec)
        .expect("Failed to serialize Savedapp vector to JSON");

        fs::write("src/saved_apps.json", json_string).expect("Failed to write JSON to file"); // rewrites whole file, not sure if more efficient way
   
        // then go back to menu
        print!("{esc}c", esc = 27 as char);
        println!("New app successfully saved, taking you back to main menu");
        display_menu(saved_app_vec, pin);
    }
    else {
        // call back display menu
        print!("{esc}c", esc = 27 as char);
        println!("New app canceled, taking you back to main menu");
        display_menu(saved_app_vec, pin);
    }
}

fn main() {
    let data = fs::read_to_string("src/saved_apps.json").expect("Failed to read json"); // read from json to &str
    let saved_app_vec: Vec<Savedapp>= serde_json::from_str(&data).expect("Failed to parse json into savedapp");

    let mut input: String = String::new();
    println!("Please enter your pin:");
    io::stdin().read_line(&mut input).expect("Failed to read pin"); 
    let pin: u64 = input.trim().parse().expect("Invalid input"); // read_line() reads line of input including \n, so need trim to get rid of newline before converting to u32
    // save var pin for seed for rng
    
    print!("{esc}c", esc = 27 as char);
    println!("Your pin: {}", pin);
    display_menu(saved_app_vec, &pin);

}
