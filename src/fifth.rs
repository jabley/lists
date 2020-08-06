use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // DANGER DANGER
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });
        let raw_tail: *mut _ = &mut *new_tail;
        // .is_null checks for null, equivalent to checking for None
        if !self.tail.is_null() {
            // If the old tail existed, update it to point to the new tail
            (*self.tail).next = Some(new_tail);
        } else {
            // Otherwise, update the head to point to it
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }
}
