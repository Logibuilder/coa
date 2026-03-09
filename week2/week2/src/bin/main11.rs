struct Node<'a> {
    value: i32,
    next: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(value: i32) -> Node<'a> {
        Node { value, next: None }
    }
    fn push(&'a mut self, node: &'a Node<'a>) {
        self.next = Some(node);
    }
    fn print(&self) {
        println!("{}", self.value);
        if let Some(n) = self.next {
            n.print()
        }
    }
}

fn main() {
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);

    node1.print();
    node2.print();
    node3.print();

    
}