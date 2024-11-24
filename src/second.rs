pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T: std::fmt::Display> List<T> {
    pub fn print_list(&mut self) {
        let mut p = &self.head;
        while let Some(ref node) = p {
            print!("{} -> ", node.elem);
            p = &node.next;
        }
        println!("null");
    }

    pub fn new() -> Self {
        List { head: Option::None }
    }
    
    pub fn push(&mut self, _elem: T) {
        let new_node = Box::new(Node {
            elem: _elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut p = self.head.take();
        while let Some(mut node) = p {
            p = node.next.take();
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

        mylist.push(33);
        mylist.push(97);
        assert_eq!(mylist.peek(), Some(&97));
        assert_eq!(mylist.peek(), Some(&97));
    }
}

