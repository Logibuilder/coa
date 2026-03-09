struct Foo {
    value: u8
}

fn print_foo(foo: &Foo) {
    println!("I am foo: {}", foo.value);
}

fn main() {
    let f = Foo { value: 4 };
    let g = &f;
    print_foo(&f);
    print_foo(g);
}