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
    pub fn print_list(&mut self) {
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

impl Drop for List {
    fn drop(&mut self) {
        let mut p = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut node) = p {
            p = mem::replace(&mut node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut mylist = List::new();
        assert_eq!(mylist.pop(), None);
        mylist.push(1);
        assert_eq!(mylist.pop(), Some(1));
        mylist.push(0);
        mylist.push(2);
        mylist.push(3);
        mylist.push(4);
        mylist.pop();
        mylist.pop();
        mylist.pop();
        assert_eq!(mylist.pop(), Some(0));
        mylist.pop();
        mylist.pop();
        mylist.pop();
        mylist.pop();
        assert_eq!(mylist.pop(), None);
    }
}

