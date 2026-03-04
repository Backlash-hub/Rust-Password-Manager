mod crypto;

use std::{str, fs, io, path::Path};
use serde::{Deserialize, Serialize};
use chrono::Local;

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    name: String,
    nonce: String,
    password: String,
    created_by_date: String,
    updated_by_date: String,
}

fn menu() {
    println!();
    println!("*** MENU ***");
    println!("1. New Password");
    println!("2. Search Password");
    println!("3. Remove Password");
    println!("4. Change Password");
    println!("5. Quit");
    println!();
}

fn get_choice() -> Option<i32> {
    println!();
    println!("Select a option (1-5): ");

    let mut choice = String::new();

    io::stdin().read_line( & mut choice ).expect("Failed to read line");
    let choice = choice.trim();

    if choice.is_empty() {
        return None;
    }

    choice.parse::<i32>().ok()
}

fn chose_task(choice: i32) -> bool {
    match choice {
        1 => {
            add_password();
            true
        }
        2 => {
            let path = "output.json";
            if !Path::new(path).exists() {
                println!();
                println!("No vault file found.");
                true
            } else {
                search_password();
                true
            }
        }
        3 => {
            remove_password();
            true
        }
        4 => {
            change_password();
            true
        }
        5 => {
            exit();
            false
        }
        _ => {
            println!("Invalid choice");
            true
        }
    }
}


fn add_password() {
    let mut choice_name:String = String::new();
    let mut choice_password:String = String::new();
    let created_by_date = Local::now();
    let updated_by_date = Local::now();

    println!("Enter your password name: ");
    io::stdin().read_line( & mut choice_name ).expect("Failed to read line");
    let choice_name = choice_name.trim();

    println!("Enter your password: ");
    io::stdin().read_line( & mut choice_password ).expect("Failed to read line");
    let choice_password = choice_password.trim();
    let key = crypto::load_or_create_key();
    let (nonce_hex, ciphertext) = crypto::encrypt_password(choice_password, &key);

    let new_password = Entry {
        name: String::from(choice_name),
        nonce: nonce_hex,
        password: ciphertext,
        created_by_date: created_by_date.to_string(),
        updated_by_date: updated_by_date.to_string(),
    };

    let path = "output.json";

    let mut entries: Vec<Entry> = if Path::new(path).exists() {
        let file_contents = fs::read_to_string(path).expect("Failed to read file");

        if file_contents.trim().is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(&file_contents).expect("Failed to parse JSON")
        }
    } else {
        Vec::new()
    };

    entries.push(new_password);

    let json = serde_json::to_string_pretty(&entries).expect("Failed to convert to JSON");

    fs::write(path, json).expect("Failed to write file");
}
fn search_password() {
    println!();
    println!("Enter your password name: ");
    println!();

    let mut choice_name = String::new();
    io::stdin()
        .read_line(&mut choice_name)
        .expect("Failed to read line");

    let choice_name = choice_name.trim();
    let path = "output.json";

    let file_contents = fs::read_to_string(path).expect("Failed to read file");

    if file_contents.trim().is_empty() {
        println!("No passwords in vault");
        return;
    }

    let entries: Vec<Entry> =
        serde_json::from_str(&file_contents).expect("Failed to parse JSON");

    let found_entry = entries.iter().find(|entry| entry.name == choice_name);

    match found_entry {
        Some(entry) => {
            let key = crypto::load_or_create_key();

            let decrypted_pass =
                crypto::decrypt_password(&entry.nonce, &entry.password, &key);

            println!("Password name: {}", entry.name);
            println!("Password: {}", decrypted_pass);
            println!("Created: {}", entry.created_by_date);
            println!("Updated: {}", entry.updated_by_date);
        }
        None => {
            println!("Password not found");
        }
    }
}

fn remove_password() {
    println!("REMOVE")
}

fn change_password() {
    println!("CHANGE")
}

fn exit() {
    println!();
    println!("Shutting down.....");
    println!("Thank you for using the vault!");
}

fn main() {
    loop {
        menu();

        let choice = match get_choice() {
            Some(num) => num,
            None => {
                println!("Please enter a number from 1 to 5.");
                continue;
            }
        };

        let keep_going = chose_task(choice);

        if !keep_going {
            break;
        }
    }
}
