use std::io::Write;

use linked_list::{show_list, BoxSingleLinkedList, RcSingleLinkedList};

fn main() {
    let mut input = String::new();
    println!("1. Single Linked List (Box)");
    println!("2. Single Linked List (Rc & RefCell)");
    print!("Select a Option: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "1" => show_list::<i32, BoxSingleLinkedList<i32>>(),
        "2" => show_list::<i32, RcSingleLinkedList<i32>>(),
        _ => {}
    }
}
