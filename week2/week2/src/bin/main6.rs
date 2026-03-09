fn total_len(a: &str, b: &str) -> usize {
    a.len() + b.len()
}
fn main() {
    let len = total_len("Hello", "World");
    println!("{len}");
}