use std::{str, fs, io, path::Path};
use serde::{Deserialize, Serialize};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use aes_gcm::aead::rand_core::RngCore;

fn menu() {
    println!("1. New Password");
    println!("2. Search Password");
    println!("3. Remove Password");
    println!("4. Change Password");
    println!("5. Quit");
}

fn get_choice() -> i32 {
    println!();
    println!("Select a option (1-5): ");
    let mut choice = String::new();
    io::stdin().read_line( & mut choice ).expect("Failed to read line");
    choice.trim().parse::<i32>().expect("Please type a number!")
}

fn chose_task(choice: i32) -> bool {
    match choice {
        1 => {
            add_password();
            true
        }
        2 => {
            search_password();
            true
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

fn load_or_create_key() -> [u8; 32] {
    let key_path = "secret.key";
    if Path::new(&key_path).exists() {
        
    }
}

fn encrypt_password(plain_text: &str, key: &[u8; 32]) -> (String, String) {
    let cipher = Aes256Gcm::new_from_slice(key).expect("invalid key");

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plain_text.as_bytes())
        .expect("encryption failed");

    (hex::encode(nonce_bytes), hex::encode(ciphertext))
}

fn add_password() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Entry {
        name: String,
        password: String,
    }

    let mut choice_name:String = String::new();
    let mut choice_password:String = String::new();

    println!("Enter your password name: ");
    io::stdin().read_line( & mut choice_name ).expect("Failed to read line");
    let choice_name = choice_name.trim();

    println!("Enter your password: ");
    io::stdin().read_line( & mut choice_password ).expect("Failed to read line");
    let choice_password = choice_password.trim();

    let new_password = Entry {
        name: String::from(choice_name),
        password: String::from(choice_password),
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
    println!("SEARCH")
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
        let keep_going = chose_task(get_choice());

        if !keep_going {
            break;
        }
    }
}
