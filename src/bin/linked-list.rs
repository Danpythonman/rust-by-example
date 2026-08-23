use std::fmt::Write;

trait LinkedList {
    fn new() -> Self;
    fn prepend(self, elem: u32) -> Self;
    fn len(&self) -> u32;
    fn stringify(&self) -> String;
}

enum Node {
    Cons(u32, Box<Node>),
    Nil,
}

struct IterativeLinkedList {
    head: Node,
}

struct RecursiveLinkedList {
    head: Node,
}

impl LinkedList for Node {
    fn new() -> Node {
        Node::Nil
    }

    fn prepend(self, elem: u32) -> Node {
        // `Cons` also has type List
        Node::Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            // And it's a non-tail recursive call which may cause stack overflow for long lists.
            Node::Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Node::Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Node::Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            },
            Node::Nil => {
                format!("Nil")
            },
        }
    }
}

impl LinkedList for RecursiveLinkedList {
    fn new() -> RecursiveLinkedList {
        RecursiveLinkedList { head: Node::new() }
    }

    fn prepend(self, elem: u32) -> Self {
        RecursiveLinkedList { head: self.head.prepend(elem) }
    }

    fn len(&self) -> u32 {
        self.head.len()
    }

    fn stringify(&self) -> String {
        self.head.stringify()
    }
}

impl LinkedList for IterativeLinkedList {
    fn new() -> IterativeLinkedList {
        IterativeLinkedList { head: Node::new() }
    }

    fn prepend(mut self, elem: u32) -> Self {
        self.head = self.head.prepend(elem);
        self
    }

    fn len(&self) -> u32 {
        let mut counter: u32 = 0u32;
        let mut curr: &Node = &self.head;
        loop {
            match curr {
                Node::Cons(_, next) => {
                    counter += 1u32;
                    curr = next;
                },
                Node::Nil => {
                    break;
                },
            }
        }
        counter
    }

    fn stringify(&self) -> String {
        let mut s: String = String::new();
        let mut curr: &Node = &self.head;
        loop {
            if !s.is_empty() {
                s.push_str(", ");
            }
            match curr {
                Node::Cons(value, next) => {
                    write!(s, "{}", value).unwrap();
                    curr = next;
                },
                Node::Nil => {
                    s.push_str("Nil");
                    break;
                },
            }
        }
        s
    }
}

fn main() {
    // Create an empty linked list
    let mut list = RecursiveLinkedList::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("recursive linked list has length: {}", list.len());
    println!("{}", list.stringify());

    // Create an empty linked list
    let mut list = IterativeLinkedList::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("iterative linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
