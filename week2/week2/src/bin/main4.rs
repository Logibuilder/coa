struct Foo {
    value: u8
}

fn get_nth_item(values: &[Foo; 4], idx: usize) -> Option<&Foo> {
    values.get(idx)
    
}

fn main() {
    
    let values = [
        Foo { value: 0},
        Foo { value: 1},
        Foo { value: 2},
        Foo { value: 3},
    ];
    match get_nth_item(&values, 1) {
        Some(val) => println!("The second value is {}", val.value),
        None => println!("Index out of bounds"),
    }
}
