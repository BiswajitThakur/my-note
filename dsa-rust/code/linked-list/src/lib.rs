mod box_single_linked_list;

use std::{
    fmt,
    io::{self, Write},
    str::FromStr,
};

pub use box_single_linked_list::LinkedList as SingleLinkedListBox;

pub trait LinkedListT {
    type Item: fmt::Display + PartialEq;
    fn new() -> Self;
    fn add(&mut self, value: Self::Item);
    fn push_back(&mut self, value: Self::Item);
    fn remove(&mut self, value: Self::Item);
    fn update(&mut self, old_val: Self::Item, new_val: Self::Item);
    fn print(&self);
}

pub fn show_list<U, T>()
where
    T: LinkedListT<Item = U>,
    U: FromStr + fmt::Display + PartialEq,
{
    let mut list = T::new();
    loop {
        println!("--- Linked List CLI ---");
        println!("1. Add node");
        println!("2. Push Back");
        println!("3. Remove node");
        println!("4. Update node");
        println!("5. Print list");
        println!("6. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().parse::<u8>().unwrap_or(0);

        match choice {
            1 => {
                print!("Enter value to add: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(value) = input.trim().parse::<U>() {
                    list.add(value);
                } else {
                    println!("Invalid input. Please enter an integer.");
                }
            }
            2 => {
                print!("Enter value to add: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(value) = input.trim().parse::<U>() {
                    list.push_back(value);
                } else {
                    println!("Invalid input. Please enter an integer.");
                }
            }
            3 => {
                print!("Enter value to remove: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(value) = input.trim().parse::<U>() {
                    list.remove(value);
                } else {
                    println!("Invalid input. Please enter an integer.");
                }
            }
            4 => {
                print!("Enter value to update: ");
                io::stdout().flush().unwrap();
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(old_value) = input.trim().parse::<U>() {
                    print!("Enter new value: ");
                    io::stdout().flush().unwrap();
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    if let Ok(new_value) = input.trim().parse::<U>() {
                        list.update(old_value, new_value);
                    } else {
                        println!("Invalid input. Please enter an integer.");
                    }
                } else {
                    println!("Invalid input. Please enter an integer.");
                }
            }
            5 => {
                println!("\nLinked List:");
                list.print();
            }
            6 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid option.");
            }
        }
        print!("Press Enter to Continue...");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();
        #[cfg(not(target_os = "windows"))]
        std::process::Command::new("clear").status().unwrap();
        #[cfg(target_os = "windows")]
        std::process::Command::new("cls").status().unwrap();
    }
}
