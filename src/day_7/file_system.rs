use std::{borrow::Borrow, cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
    children: RefCell<Option<Vec<Rc<File>>>>,
}

pub fn main(lines: Vec<String>) {
    // Problem 1: Build a file tree
    // Problem 2: Sum up filesizes
}
