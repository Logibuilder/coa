fn total_len(a: &str, b: &str) -> usize {
    a.len() + b.len()
}
fn main() {
    let len = total_len("Hello", "World");
    let len_bis = len;
    println!("{len}");
    println!("{len_bis}");

}