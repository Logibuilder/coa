struct Foo {
    value: u8
}

impl Foo {
    fn new() -> Foo {
        Foo {value: 0}
    }
    fn change(&mut self) {
        self.value += 13;
    }
    fn print(&self) {
        println!("Foo: {}", self.value);
    }
}

fn main() {
    let mut f = Foo::new();

    f.change();
    
    let g = &f;
    
    g.print();
}