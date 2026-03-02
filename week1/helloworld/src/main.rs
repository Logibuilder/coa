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
    check_password(&passworld.trim());
}


pub fn check_password(passworld: &str) { 
    let correct_password = "secret";
    
    if passworld == correct_password {
        println!("Good");
    } else {
        println!("Wrong");
    }
}