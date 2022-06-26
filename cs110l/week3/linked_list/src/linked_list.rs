use std::fmt::{self, Display};
use std::option::Option;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value: value, next: next}
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {head: None, size: 0}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn is_empty(&self) -> bool {
        self.get_size() == 0
    }
    
    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.size += 1;
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        self.size -= 1;
        Some(node.value)
    }
}


impl<T: Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut current = &self.head;
        let mut result = String::new();
        loop {
            match current {
                Some(node) => {
                    result = format!("{} {}", result, node.value);
                    current = &node.next;
                },
                None => break,
            }
        }
        write!(f, "{}", result)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(mut node) = current {
            current = node.next.take();
        }
    }
}

impl<T: Clone> Clone for Node<T>{
    fn clone(&self) -> Self {
        Node::new(self.value.clone(), self.next.clone())
    }
}
impl<T: Clone> Clone for LinkedList<T>{
    fn clone(&self) -> Self {
        LinkedList { head: self.head.clone(), size: self.size.clone() }
    }
}

impl<T: PartialEq> PartialEq for Node<T>{
    fn eq(&self, other: &Self) -> bool {
        T::eq(&self.value, &other.value)
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T>{
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size { return false };
        let mut curr_self = &self.head;
        let mut curr_other = &other.head;
        while curr_self.is_some(){
            if curr_self.as_ref().unwrap() != curr_other.as_ref().unwrap() {
                return false;
            }
            curr_self = &curr_self.as_ref().unwrap().next;
            curr_other = &curr_other.as_ref().unwrap().next;
        }
        return true;
    }
}

impl<T> Iterator for LinkedList<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}


impl<T:Clone> Iterator for LinkedListIter<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current.as_ref();
        self.current = &node?.next;
        Some(node?.value.clone())
    }
}

impl<'a, T:Clone> IntoIterator for &'a LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter {current: &self.head}
    }
}