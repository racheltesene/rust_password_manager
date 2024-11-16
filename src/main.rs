use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, Write, Read};
use std::path::Path;

// PasswordManager struct with HashMap to store passwords
struct PasswordManager {
    data: HashMap<String, String>,
}

impl PasswordManager {
    // Create a new PasswordManager
    fn new() -> Self {
        PasswordManager {
            data: HashMap::new(),
        }
    }

    // Add a new password entry
    fn add_entry(&mut self, service: String, password: String) {
        self.data.insert(service, password);
        println!("Password added successfully!");
    }

    // Retrieve a password by service name
    fn get_entry(&self, service: &String) {
        match self.data.get(service) {
            Some(password) => println!("Password for {}: {}", service, password),
            None => println!("No entry found for {}", service),
        }
    }

    // Delete a password entry
    fn delete_entry(&mut self, service: &String) {
        if self.data.remove(service).is_some() {
            println!("Password for {} deleted.", service);
        } else {
            println!("No entry found for {}", service);
        }
    }

    // Save the data to a file
    fn save_to_file(&self, file_path: &str) {
        let serialized = serde_json::to_string(&self.data).expect("Serialization failed");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .expect("Unable to open file");
        file.write_all(serialized.as_bytes())
            .expect("Write to file failed");
        println!("Data saved to {}", file_path);
    }

    // Load data from a file
    fn load_from_file(&mut self, file_path: &str) {
        if Path::new(file_path).exists() {
            let mut file = OpenOptions::new()
                .read(true)
                .open(file_path)
                .expect("Unable to open file");
            let mut content = String::new();
            file.read_to_string(&mut content)
                .expect("Failed to read file");
            self.data = serde_json::from_str(&content).expect("Deserialization failed");
            println!("Data loaded from {}", file_path);
        } else {
            println!("File {} not found. Starting fresh.", file_path);
        }
    }
}

// Helper function to display the menu
fn display_menu() {
    println!("\n--- Password Manager ---");
    println!("1. Add a new password");
    println!("2. Retrieve a password");
    println!("3. Delete a password");
    println!("4. Save to file");
    println!("5. Load from file");
    println!("6. Exit");
}

fn main() {
    let mut manager = PasswordManager::new();
    let file_path = "passwords.json";

    // Load existing data (if available)
    manager.load_from_file(file_path);

    loop {
        display_menu();
        println!("Enter your choice: ");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter service name: ");
                let mut service = String::new();
                io::stdin().read_line(&mut service).expect("Failed to read input");

                println!("Enter password: ");
                let mut password = String::new();
                io::stdin().read_line(&mut password).expect("Failed to read input");

                manager.add_entry(service.trim().to_string(), password.trim().to_string());
            }
            "2" => {
                println!("Enter service name: ");
                let mut service = String::new();
                io::stdin().read_line(&mut service).expect("Failed to read input");

                manager.get_entry(&service.trim().to_string());
            }
            "3" => {
                println!("Enter service name: ");
                let mut service = String::new();
                io::stdin().read_line(&mut service).expect("Failed to read input");

                manager.delete_entry(&service.trim().to_string());
            }
            "4" => manager.save_to_file(file_path),
            "5" => manager.load_from_file(file_path),
            "6" => {
                println!("Exiting Password Manager. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
