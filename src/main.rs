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

#[test]
fn char_type() {
    let a = 'a';
    let b: char = 'b';
    println!("a {} b {}", a, b);
}

#[test]
fn tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);
    
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("a {} b {} c {}", a, b, c);
    
    let (a, _, c) = data; // destructure
    println!("a {} c {}", a, c);
    
    data.0 = 20;
    println!("{:?}", data);
}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);
    
    let test = ();
    println!("{:?}", test);
}

#[test]
fn array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);
    
    let a = array[0];
    let b = array[1];
    println!("a {} b {}", a, b);
    
    array[0] = 10;
    array[2] = 30;
    println!("{:?}", array);
    
    let length: usize = array.len();
    println!("length {}", length);
}

#[test]
fn two_dimensional_array() {
    let matrix: [[i32; 2]; 2] = [
        [1, 2],
        [3, 4]
    ];
    
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[0][1]);
    println!("{:?}", matrix);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    const MINIMUM: i32 = 10;
    println!("Maximum {}", MAXIMUM);
    println!("Minimum {}", MINIMUM);
}

#[test]
fn variable_scope() {
    let hutomo = 1;

    { // inner scope
        println!("inner hutomo {}", hutomo);
        let sakti = 2;
        println!("inner sakti {}", sakti);
    }
    
    // println!("outer hutomo {} sakti {}", hutomo, sakti);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Hutomo");
    println!("a {} b {}", a, b);
}

fn function_b() {
    let a = 20;
    let b = String::from("Sakti");
    println!("a {} b {}", a, b);
}

#[test]
fn string() {
    let name: &str = "   Hutomo   ";
    let trim: & str = name.trim();
    println!("name {}", name);
    println!("trim {}", trim);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Hutomo Sakti");
    println!("{}", name);
    
    name.push_str(" Kartiko");
    println!("{}", name);
    
    let kartiko = name.replace("Hutomo", "Kartiko");
    println!("{}", kartiko);
}

#[test]
fn ownership_rules() {
    // a tidak bisa diakses di sini, belum dideklarasikan
    let a = 10; // a bisa diakses mulai di sini

    { // b tidak bisa diakses di sini, belum dideklarasikan
        let b = 20;
        println!("b {}", b);
    } // scope b selesai, b dihapus, b tidak bisa diakses
    
    println!("a {}", a);
} // scope a selesai, a dihapus, a tidak bisa diakses

#[test]
fn data_copy() {
    let a = 10;
    let b = a;
    
    println!("a  {} b {}", a, b);
}

#[test]
fn ownership_movement() {
    let name1 = String::from("Hutomo");
    
    // ownership dari name1 dipindahkan ke name2
    let name2 = name1;
    // name1 tidak bisa diakses di sini
    
    println!("name2 {}", name2);
}

#[test]
fn clone() {
    let name1 = String::from("Hutomo");
    let name2 = name1.clone();
    
    println!("name1 {}", name1);
    println!("name2 {}", name2);
}

#[test]
fn if_expression() {
    let value = 90;
    
    let result: &str = if value >= 80 {
        "Good"
    } else if value >= 60 {
        "Not Bad"
    } else {
        "Very Bad"
    };
    
    println!("result {}", result);
}

#[test]
fn loop_expression() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }
        
        println!("counter: {}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut counter = 10;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2
        }
    };
    
    println!("result {}", result);
}

#[test]
fn loop_label() {
    let mut number = 1;
    'outer: loop {
        let mut i = 0;
        loop {
            if number > 10 {
                break 'outer;
            }
            
            println!("{} x {} = {}", number, i, number * i);
            i += 1;
            if i > 10 {
                break;
            }
        }
        
        number += 1;
    }
}

#[test]
fn while_loop() {
    let mut counter = 0;
    while counter <= 10 {
        if counter % 2 == 0 {
            println!("counter: {}", counter);
        }
        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [&str; 5] = ["a", "b", "c", "d", "e"];
    let mut index = 0;
    
    while index < array.len() {
        println!("value {}", array[index]);
        index += 1;
    }
}

#[test]
fn array_iteration_for_loop() {
    let array: [&str; 5] = ["a", "b", "c", "d", "e"];
    
    for value in array {
        println!("value {}", value);
    }
}

#[test]
fn range_exclusive() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    
    let range = 0..5;
    println!("start {}", range.start);
    println!("end {}", range.end);

    for i in  range {
        println!("array {}", array[i]);
    }
}

#[test]
fn range_inclusive() {
    let  array: [i32; 5] = [1, 2, 3, 4, 5];
    
    let range = 0..=4;
    println!("start {}", range.start());
    println!("end {}", range.end());

    for i in  range {
        println!("array {}", array[i]);
    }
}

fn say_hello() {
    println!("Hello");
}

#[test]
fn test_say_hello() {
    say_hello();
}

fn say_goodbye(name: &str) {
    println!("Goodbye {}", name);
}

#[test]
fn test_say_goodbye() {
    say_goodbye("Hutommo");
    say_goodbye("Sakti");
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }
    
    let mut  result = 1;
    for i in 1..=n {
        result *= i;
    }
    
    result
}

#[test]
fn test_factorial_loop() {
    let result  = factorial_loop(5);
    println!("factorial_loop {}", result);
    
    let  result  = factorial_loop(-10);
    println!("factorial_loop {}", result);
}

fn print_text(value: String, times: i32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }
    
    print_text(value, times - 1)
}

#[test]
fn test_print_text() {
    print_text(String::from("Hello"), 5);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }
    
    n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_recursive(5);
    println!("factorial_recursive {}", result);
}

fn print_number(number: i32) {
    println!("number {}", number);
}

fn hi(name: String) {
    println!("Hi {}", name);
}

#[test]
fn test_hi() {
    let number = 20;
    print_number(number);
    println!("{}", number);

    let name = String::from("Hutomo");
    hi(name);
    // println!("{}", name);
}

fn full_name(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_full_name() {
    let first_name = String::from("Hutomo");
    let last_name = String::from("Sakti");

    let full_name = full_name(&first_name, &last_name);
    println!("Name {}", full_name);
    println!("first_name {}", first_name);
    println!("last_name {}", last_name);
}

fn change_value(value: &mut String) {
    value.push_str(" Sakti");
}

#[test]
fn test_change_value() {
    let mut name = String::from("Hutomo");
    change_value(&mut name);
    change_value(&mut name);
    change_value(&mut name);
    println!("Name {}", name);
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);

    name
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Hutomo");
    let last_name = String::from("Sakti");

    let full_name = get_full_name(&first_name, &last_name);
    println!("Name {}", full_name);
}

#[test]
fn slice_reference() {
    let array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let slice1: &[i32] = &array[..];
    println!("slice1 {:?}", slice1);

    let slice2: &[i32] = &array[0..5];
    println!("slice2 {:?}", slice2);

    let slice3: &[i32] = &array[5..];
    println!("slice3 {:?}", slice3);
}

#[test]
fn string_slice() {
    let name: String = String::from("Hutomo Sakti Kartiko");

    let first_name: &str = &name[0..6];
    println!("first_name {}", first_name);

    let last_name: &str = &name[13..];
    println!("last_name {}", last_name);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello, {} my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!("first_name {}", person.first_name);
    println!("middle_name {}", person.middle_name);
    println!("last_name {}", person.last_name);
    println!("age {}", person.age);
}

#[test]
fn struct_person() {
    let first_name = String::from("Hutomo");
    let last_name = String::from("Kartiko");
    let person: Person = Person {
        first_name,
        middle_name: String::from("Sakti"),
        last_name,
        age: 20,
    };
    print_person(&person);

    let person2: Person = Person{
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };
    print_person(&person2);
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn tupple_struct() {
    let geo_point = GeoPoint(-6.00001, 100.00001);
    println!("long {}", geo_point.0);
    println!("lat {}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing1 = Nothing;
    let _nothing2 = Nothing {};
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Hutomo"),
        middle_name: String::from("Sakti"),
        last_name: String::from("Kartiko"),
        age: 20,
    };

    person.say_hello("Budi");
}

#[test]
fn test_method_new() {
    let geo_point = GeoPoint::new(-6.00001, 100.00001);
    println!("long {}", geo_point.0);
    println!("lat {}", geo_point.1);
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level = Level::Premium;
    match level {
        Level::Regular => {
            println!("Regular");
        }
        Level::Premium => {
            println!("Premium");
        }
        Level::Platinum => {
            println!("Platinum");
        }
    }
}

enum Payment {
    // card number
    CreditCard(String),
    // bank name, account number
    BankTransfer(String, String),
    // ewallet name, ewallet nummber
    EWallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, account) => {
                println!("Paying with bank transfer {} account {} amount {}", bank, account, amount);
            }
            Payment::EWallet(wallet, number) => {
                println!("Paying with ewallet {} number {} amount {}", wallet, number, amount);
            }
        }
    }
}

#[test]
fn test_payment() {
    let _payment1 = Payment::CreditCard(String::from("2342342342"));
    _payment1.pay(10000);

    let _payment2 = Payment::BankTransfer(String::from("BCA"), String::from("2342342342"));
    _payment2.pay(10000);

    let _payment3 = Payment::EWallet(String::from("GoPay"), String::from("2342342342"));
    _payment3.pay(10000);
}

#[test]
fn test_match_value() {
    let name: &str = "Hutomo";

    match name {
        "Hutomo" | "Sakti"=> {
            println!("Hello Bos");
        }
        other => {
            println!("Hello {}", other);
        }
    }
}

#[test]
fn test_range_value() {
    let value = 80;

    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Bad")
        }
        other => {
            println!("Invalid value {}", other);
        }
    }
}

#[test]
fn test_struct_patterns() {
    let geo_point = GeoPoint::new(100.0, 100.0);
    
    match geo_point { 
        GeoPoint(long, 0.0) => {
            println!("long {}", long);
        }
        GeoPoint(0.0, lat) => {
            println!("lat {}", lat);
        }
        GeoPoint(long, lat) => {
            println!("long {} lat {}", long, lat);
        }
    }
    
    let person = Person {
        first_name: String::from("Hutomo"),
        middle_name: String::from("Sakti"),
        last_name: String::from("Kartiko"),
        age: 20,
    };
    
    match person { 
        Person { first_name, last_name, ..} => {
            println!("first_name {} last_name {}", first_name, last_name);
        }
    }
}


#[test]
fn test_ignoring() {
    let geo_point = GeoPoint::new(100.0, 100.0);

    match geo_point {
        GeoPoint(long, _) => {
            println!("long {}", long);
        }
    }
}

#[test]
fn test_ignoring_range() {
    let value = 101;
    
    match value {
        75..=100 => {
            println!("Great")
        }
        50..=74 => {
            println!("Good")
        }
        25..=49 => {
            println!("Not Bad")
        }
        0..=24 => {
            println!("Bad")
        }
        _ => {
            println!("Invalid value");
        }
    }
}

#[test]
fn test_match_expression() {
    let value: i32 = 10;
    let result = match value {
        10 => "Ten",
        20 => {
            "Twenty"
        },
        _ => "Other",
    };
    
    println!("result {}", result);
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

#[test]
fn test_tyoe_alias() {
    let cutomer = Customer {
        id: String::from("1213123"),
        name: String::from("Hutomo"),
        age: 20,
    };
    
    println!("id {} name {} age {}", cutomer.id, cutomer.name, cutomer.age);
}