use std::io;

fn main() {
    println!("Fahrenheit to Celsius converter");
    println!("Please enter a temperature in Fahrenheit:");

    let mut temperature_f = String::new();

    io::stdin()
        .read_line(&mut temperature_f)
        .expect("Failed to read line");

    let temperature_f = temperature_f.trim().parse::<i32>().unwrap();
    let temperature_c = (temperature_f - 32) * 5 / 9;

    println!("The temperature in Celsius is: {}", temperature_c)
}
