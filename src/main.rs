use std::fmt;

fn main() {
    println!("Hello, world!");
    let mut n1: Node = Node::new(1);
    
    let mut n2: Node = Node::new(2);
    println!("{}", n1);
    println!("{}", n2);
    n1.next = Some(Box::new(n2));

}

#[derive(PartialEq, PartialOrd, Debug)]
struct Node{
    value: i32,
    next: Option<Box<Node>>
}

impl Node{
    fn new(value: i32) -> Node{
        Node{value, next: None}
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

