use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn print_list(&self) {
        let mut p = &self.head;
        while let Link::More(ref node) = p {
            print!("{} -> ", node.elem);
            p = &node.next;
        }
        println!("null");
    }

    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    
    pub fn push(&mut self, _elem: i32) {
        let new_node = Box::new(Node {
            elem: _elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}
