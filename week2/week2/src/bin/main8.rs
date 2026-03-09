fn longuest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b 
    }
}

fn main() {
    let a = String::from("Hello");
    let b = "Rust World";

    println!("longuest: {}", longuest(&a, &b));
}