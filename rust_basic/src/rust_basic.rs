use std::{io::Bytes, mem::size_of_val};

pub fn rust_basic(){
    let a: char = 'a';
    println!("{} ", size_of_val(&a));

    let f: bool = true;
    let t: bool = true && true;

    assert_eq!(t,f);

    let x:u32 = 5u32;

    let y:u32 = {
        let x_squared = x*x;
        let x_cube = x_squared * x;

        // This expression will be assigned to 'y'
        x_cube + x_squared + x
    };

    let z = {
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    println!("{:?}",get_option(2));

    stack_memory();

    deep_copy();

    ownership();
}

// Match Like other language is switch
fn get_option(tp: u8) {
    match tp {
        1 => 22,
        2 => 33,
        3 => 44,
        _ => 55,
    };
}

// Stack Memory
fn stack_memory (){
    let (x,y) = (42,10);
    let z = add_numbers(x, y);
    println!("The result is {}",z)
}

fn add_numbers(a: i32, b:i32) -> i32 {
    a + b
}

// Deep Copy
fn deep_copy(){
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("S1 = {}, S2 = {}", s1, s2);
}

// Ownership
fn ownership(){
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    println!("{} {}", s1, s2);
}