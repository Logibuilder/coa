fn main() {
    println!("Hello, world!");
    let x = 42;
    println!("val de x = {}", x);
    eprintln!("val de x = {:x}", x);
    eprintln!("val de x = {:X}", x);

    let mut y = 1;
    while y<=10 {
        println!("y = {}", y);
        y += 1;
    }

    let mut passworld = String::new();
    println!("Enter your password: ");
    std::io::stdin().read_line(&mut passworld).expect("Error reading password");

    let password = passworld.trim();
    match check_password(password) {
        Ok(_) => println!("Good!"),
        Err(e) => println!("Error: {:?}", e),
    }
    
}

#[derive(Debug)]
enum Error {
    WrongPassword
}

pub fn check_password(passworld: &str) -> Result<(), Error> { 
    
    match passworld {
        "secret" => {
            Ok(())
        },
        _ => Err(Error::WrongPassword)
    }
}