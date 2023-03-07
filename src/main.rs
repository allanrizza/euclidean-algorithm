use std::{io::{self, stdin, stdout, Read, Write}};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    println!("\nFIND THE GREATEST COMMON DIVISOR (GCD) WITH THE EUCLIDEAN ALGORITHM\n");

    let mut input_number_a = String::new();
    let mut input_number_b = String::new();

    println!("Type the first number: ");
    io::stdin()
    .read_line(&mut input_number_a)
    .expect("error");

    println!("\nType the second number: ");
    io::stdin()
        .read_line(&mut input_number_b)
        .expect("error");

    let mut a: i32 = input_number_a.trim().parse().expect("Input not an integer");
    let mut b: i32 = input_number_b.trim().parse().expect("Input not an integer");

    if a.is_negative() { a = a * -1 };
    if b.is_negative() { b = b * -1; }


    println!("\nGCD({}, {}) = {}\n", a, b, algoritmo(a, b));

    pause();
}

fn algoritmo(a: i32, b: i32) -> i32 {

    if b==0 {
        return a;
    } else {
        return algoritmo(b, a%b) ;
    }
}