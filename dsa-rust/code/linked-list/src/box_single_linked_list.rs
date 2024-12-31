use std::fmt;

use crate::LinkedListT;

struct Node<T: fmt::Display + PartialEq> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T: fmt::Display + PartialEq> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T: fmt::Display + PartialEq> Drop for Node<T> {
    fn drop(&mut self) {
        println!("Node with value {} has been dropped.", self.value);
    }
}

pub struct LinkedList<T: fmt::Display + PartialEq> {
    head: Option<Box<Node<T>>>,
}

impl<T: fmt::Display + PartialEq> LinkedListT for LinkedList<T> {
    type Item = T;
    fn new() -> Self {
        Self { head: None }
    }
    fn add(&mut self, value: Self::Item) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    fn push_back(&mut self, value: Self::Item) {
        let new_node = Box::new(Node::new(value));

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let mut current = &mut self.head;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }

        *current = Some(new_node);
    }
    fn remove(&mut self, value: Self::Item) {
        let mut current = &mut self.head;
        while let Some(_) = current {
            if current.as_ref().unwrap().value == value {
                let next = current.as_mut().unwrap().next.take();
                *current = next;
                return;
            } else {
                current = &mut current.as_mut().unwrap().next;
            }
        }
    }
    fn update(&mut self, old_value: Self::Item, new_value: Self::Item) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            if node.value == old_value {
                node.value = new_value;
                return;
            }
            current = &mut node.next;
        }
    }
    fn print(&self) {
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
    #[inline(always)]
    fn header(&self) -> &str {
        "--- Single Linked List (Box) CLI ---"
    }
}
