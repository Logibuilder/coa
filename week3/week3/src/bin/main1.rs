trait CaesarExt {
    /// Encode self using a Caesar's cypher with the given shift
    fn encode(&self, shift: u8) -> String;
    /// Decode self using a Caesar's cypher with the given shift
    fn decode(&self, shift: u8) -> String;
}

// ...
fn main() {
    let encoded = "Hello world~❤️".encode(13);
    let decoded = encoded.as_str().decode(13);
    // Print encoded as escaped ascii because several valid ascii are not printable
    println!("{decoded} <-> {}", encoded.as_bytes().escape_ascii().to_string());
}


impl CaesarExt for &str {
    fn encode(&self, shift: u8) -> String {
        self.chars().map( |c| shift_char(c, shift)).collect()
    }

    fn decode(&self, shift: u8) -> String {
        self.chars().map( |c| shift_char(c, 128 - shift)).collect()
    }
}


fn shift_char(c: char, shift: u8) -> char {
    if c.is_ascii() {
        let shifted = (c as u8 + shift) % 128;
        shifted as char
    } else {
        c
    }
}