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

#[test]
fn explicit() {
    let age: i32 = 20;
    println!("Age {}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("a {}", a);
    
    let b: f32 = 10.2;
    println!("b {}", b);
}

#[test]
fn number_conversion() {
    let a: i16 = 10;
    println!("a {}", a);
    
    let b: i8 = a as i8;
    println!("b {}", b);
    
    let c: i32 = b as i32;
    println!("c {}", c);
    
    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("e {}", e);
}

#[test]
fn numberic_operator() {
    let a = 10;
    let b = 20;
    let c = a + b;
    let d = a * b;
    let e = a / b;
    let f = a % b;
    println!("c {} d {} e {} f {}", c, d, e, f);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("a {}", a);
    
    a += 10;
    println!("a {}", a);
    
    a *= 10;
    println!("a {}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;
    println!("a {} b {}", a, b);
}

#[test]
fn comparasion() {
    let result = 10 > 20;
    println!("result {}", result);
}

#[test]
fn boolean_operator() {
    let absen = 70;
    let nilai_akhir = 80;
    
    let lulus = absen >= 70;
    let lulus_nilai_akhir = nilai_akhir >=75;
    
    let lulus_final = lulus && lulus_nilai_akhir;
    println!("lulus {}", lulus_final);
}