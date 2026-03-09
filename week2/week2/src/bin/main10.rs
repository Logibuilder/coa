struct Message<'a> {
    content: &'a str,
}

impl<'a> Message<'a> {
    fn from(s: &'a str) -> Message {
        Message {
            content: s
        }
    }
}
fn main() {
    let s = String::from("Hello");
    let m = Message::from(&s);
    println!("{}", m.content);
}
