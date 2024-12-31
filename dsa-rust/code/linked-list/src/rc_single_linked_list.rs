use std::{cell::RefCell, fmt, rc::Rc};

use crate::LinkedListT;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T: fmt::Display + PartialEq> {
    value: T,
    next: Link<T>,
}

pub struct LinkedList<T: fmt::Display + PartialEq> {
    head: Link<T>,
}

impl<T: fmt::Display + PartialEq> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Dropping node with value: {}", self.value);
    }
}

impl<T: fmt::Display + PartialEq> LinkedListT for LinkedList<T> {
    type Item = T;
    fn new() -> Self {
        Self { head: None }
    }
    fn add(&mut self, value: Self::Item) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }
    fn push_back(&mut self, value: Self::Item) {
        todo!()
    }
    fn remove(&mut self, value: Self::Item) {
        todo!()
    }
    fn update(&mut self, old_val: Self::Item, new_val: Self::Item) {
        todo!()
    }
    fn print(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} -> ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
    fn header(&self) -> &str {
        "--- Single Linked List (Rc & RefCell) CLI ---"
    }
}
