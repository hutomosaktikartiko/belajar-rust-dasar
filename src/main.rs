fn main() {
    println!("Hello, world!");
    
    println!("Hello, again!")
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Hutomo";
    println!("Hello, {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Hutomo";
    println!("Hello, {}", name);
    
    name = "Sakti";
    println!("Hello, {}", name);
}

#[test]
fn shadowing() {
    let name = "Hutomo";
    println!("Hello, {}", name);
    
    let name = 10;
    println!("Hello, {}", name);
}