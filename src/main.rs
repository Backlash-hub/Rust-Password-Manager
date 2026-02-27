use std::io;
use serde::Serialize;

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

fn add_password() {
    #[derive(Serialize)]
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

    let json_string = serde_json::to_string(&new_password).unwrap();
    println!("Compact JSON: {}", json_string);
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
    println!("QUIT");
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
