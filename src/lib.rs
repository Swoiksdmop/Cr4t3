use std::io;
pub fn input() {
  println!("Welcome to my program of converting Fahrenheit to Celcius!!!\n");

    println!("\nWhat would you like to convert to celcius? >");

    let mut one = String::new();

    io::stdin()
        .read_line(&mut one)
        .expect("Failed to read line");

    let one: f64 = one.trim().parse::<f64>().expect("FAILED TO READ/PARSE INPUT FROM LINE!!!");

    let _conversion = (one - 32.0) * 5.0 / 9.0;

    println!("\n\n{one} degrees fahrenheit is {_conversion} degrees celcius");
}

pub fn three_tries() {
    for _ in 0..2 {
        input()
    }
}

pub fn calle() {
    println!("Success!");
}
