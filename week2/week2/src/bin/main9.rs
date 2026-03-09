fn insert<'a>(vec: &mut Vec<&'a str>, s: &'a str) {
    vec.push(s);
}

fn main() {
    let message = String::from("Hello rust world");
    let message2 = String::from("lifetimes are great");
    let mut v = Vec::new();

    insert(&mut v, &message);
    insert(&mut v, &message2);
    
    println!("{v:?}");
}
