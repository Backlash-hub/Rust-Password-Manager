use std::{str, fs, io, path::Path};
use serde::{Deserialize, Serialize};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use aes_gcm::aead::rand_core::RngCore;
use chrono::Local;

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
        let key_hex = fs::read_to_string(key_path).expect("Failed to read key file");
        let key_bytes = hex::decode(&key_hex.trim()).expect("Failed to parse hex key file");
        let key_array: [u8; 32] = key_bytes.try_into().expect("Failed to convert key file");

        key_array
    } else {
        let mut key = [0; 32];
        OsRng.fill_bytes(&mut key);
        fs::write(key_path, hex::encode(key)).expect("Failed to write key file");
        key
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

fn decrypt_password(nonce_hex: &str, ciphertext_hex: &str, key: &[u8; 32]) -> String {
    let cipher = Aes256Gcm::new_from_slice(key)
        .expect("Invalid key length");

    let nonce_vec = hex::decode(nonce_hex)
        .expect("Invalid nonce hex");
    let ciphertext = hex::decode(ciphertext_hex)
        .expect("Invalid ciphertext hex");

    let nonce = Nonce::from_slice(&nonce_vec);

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .expect("Decryption failed");

    String::from_utf8(plaintext)
        .expect("Plaintext was not valid UTF-8")
}

fn add_password() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Entry {
        name: String,
        nonce: String,
        password: String,
        created_by_date: String,
        updated_by_date: String,
    }

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
    let key = load_or_create_key();
    let (nonce_hex, ciphertext) = encrypt_password(choice_password, &key);

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
    let mut choice_name:String = String::new();
    io::stdin().read_line( & mut choice_name ).expect("Failed to read line");
    let choice_name = choice_name.trim();
    let path = "output.json";

    let file_contents = fs::read_to_string(path).expect("Failed to read file");
    if file_contents.trim().is_empty() {
        println!("No password in vault");
    } else {
        let json = serde_json::to_string_pretty(&file_contents).expect("Failed to convert to JSON");
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
        let keep_going = chose_task(get_choice());

        if !keep_going {
            break;
        }
    }
}
