struct Foo {
    value: u8
}

fn change_foo(foo: &mut Foo) {
    foo.value += 13;
}

fn main() {
    let mut f = Foo { value: 4 };
    change_foo(&mut f);
    println!("foo is now {}", f.value);
}
