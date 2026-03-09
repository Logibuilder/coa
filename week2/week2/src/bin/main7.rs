enum Message {
    Hello,
    World
}
fn message(m: Message) -> &'static str {
    match m {
        Message::Hello => "Hello",
        Message::World => "World",
    }
}

fn main() {
    println!("{}", message(Message::Hello));
}