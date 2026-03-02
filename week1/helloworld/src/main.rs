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
}
