fn main() {
    let t = ToUpper {};
    println!("{}", t.encode("helLO WOrld"));

    let t = ToLower {};
    println!("{}", t.encode("helLO WOrld"));
}


trait Ecoder {
    fn encode(&self, message : &str) -> String;
}


trait Decoder {
    fn decode(&self, message : &str) -> String;
}

struct ToUpper {

}

struct ToLower {

}

impl Ecoder for ToUpper {
    fn encode(&self, message: &str) -> String {
        message.to_uppercase()
    }
}

impl Ecoder for ToLower {
    fn encode(&self, message: &str) -> String {
        message.to_lowercase()
    }
} 