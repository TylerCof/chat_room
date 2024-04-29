#![allow(warnings)]

extern crate termion; // Import the external crate `termion`
use std::fs::{File, OpenOptions}; // Import specific modules from the standard library: File and OpenOptions
use std::io::{self, BufRead, Write}; // Import IO-related modules from the standard library
use termion::color; // Import the `color` module from the `termion` crate

// Function to read messages from a file
fn read_messages_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?; // Try to open the file specified by the file_path
    let reader = io::BufReader::new(file); // Create a buffered reader from the file

    // Read lines from the file and collect them into a vector of strings
    let messages: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    Ok(messages) // Return the vector of messages or an IO error
}

// Function to write a message to a file
fn write_message_to_file(file_path: &str, message: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)?; // Open the file in append mode

    writeln!(file, "{}", message)?; // Write the message to the file

    Ok(()) // Return Ok if successful, otherwise return an IO error
}

// Function to get user input from the command line
fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt); // Print the prompt

    let mut input = String::new(); // Create a new empty string
    io::stdin().read_line(&mut input).expect("Failed to read line"); // Read user input from stdin

    input.trim().to_string() // Trim whitespace and return the user input as a String
}

// Main function
fn main() {
    let mut username = String::new(); // Create a mutable variable to store the username
    let file_path = "/common/ChatRoomProject/messages.txt"; // Path to the message file

    // Loop to interact with the user until they choose to exit
    loop {
        println!("1. Read messages\n2. Write message\n3. Exit"); // Display menu options
        let choice: u32 = get_user_input("Enter your choice:").parse().unwrap_or(0); // Get user choice

        match choice {
            1 => {
                // Read messages from the file
                let messages = read_messages_from_file(file_path)
                    .expect("Failed to read messages from file");

                println!("--- Messages ---");
                for msg in &messages {
                    let parts: Vec<&str> = msg.splitn(2, ":").collect(); // Split message into username and content
                    if parts.len() == 2 {
                        // Set different colors for different users
                        match parts[0] {
                            "Tyler" => print!("{}", color::Fg(color::LightYellow)),
                            "Brie" => print!("{}", color::Fg(color::LightMagenta)),
                            "Test" => print!("{}", color::Fg(color::Cyan)),
							"MurderMittenz" => print!("{}", color::Fg(color::Red)),
							"Caffeine" => print!("{}", color::Fg(color::LightBlue)),
							"CrashOverride" => print!("{}", color::Fg(color::Green)),
							
							
                            _ => print!("{}", color::Fg(color::Reset)),
                        }
                        println!("{}", msg); // Print the message
                        print!("{}", color::Fg(color::Reset)); // Reset color
                    } else {
                        println!("{}", msg); // Print the message without color
                    }
                }
            }
            2 => {
                if username.is_empty() {
                    username = get_user_input("Enter your username:"); // Ask for username if it's empty
                }
                let message = get_user_input("Enter message:"); // Get the message from the user
                write_message_to_file(file_path, &format!("{}: {}", username, message))
                    .expect("Failed to write message to file"); // Write the message to the file
            }
            3 => {
                println!("Exiting the program."); // Display exit message
                break; // Exit the loop
            }
            _ => {
                println!("Invalid choice. Please enter a valid option."); // Display invalid choice message
            }
        }
    }
}
