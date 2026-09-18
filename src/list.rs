use std::fmt::Debug;


pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    item: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.item)
            }
        }
    }

    pub fn push(&mut self, item: T) {
        let node = Box::new(Node {
            item, next: self.head.take()
        });
        self.head = Some(node);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }

    pub fn append(&mut self, mut items: impl Iterator<Item=T>) {
        if let Some(item) = items.next() {
            self.append(items);
            self.push(item);
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut next = self.head.take();
        while let Some(mut node) = next {
            next = node.next.take()
        }
    }
}

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}


pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            None => None,
            Some(node) => {
                self.next = node.next.as_deref();
                Some(&node.item)
            }
        }
    }
}