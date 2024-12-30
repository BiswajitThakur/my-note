use std::io::Write;

use linked_list::{show_list, SingleLinkedListBox};

fn main() {
    let mut input = String::new();
    println!("1. Single Linked List (Box)");
    print!("Select a Option: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => show_list::<i32, SingleLinkedListBox<i32>>(),
        _ => {}
    }
}
