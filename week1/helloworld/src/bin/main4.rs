
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error reading input");

    let input = input.trim();
    let checker = PasswordChecker::new("secret");

    // Call the check function on the checker.
    // This function should have the same return type as
    // the check_password function of the previous exercise
    match checker.check_password(&input) {
        Ok(_) => println!("Good!"),
        Err(e) => println!("Error: {:?}", e),
    }
}


#[derive(Debug)]
enum Error {
    WrongPassword
}
pub struct PasswordChecker {
    secret : String,
}


impl PasswordChecker {

    fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }

    fn check_password(&self, password: &str) -> Result<(), Error> { 
        
    
        if password == self.secret {
            Ok(())
        } else {
            Err(Error::WrongPassword)
        }        
    }
}