use std::io;

fn input_number() -> f32 {
    println!("Please input your temperature in celcius");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: f32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Must enter a number!"),
    };
    return guess;
}

fn convert_celcius_to_farenheit(temp: f32) -> f32 {
    return (temp * 1.8) + 32.0;
}

fn main() {
    let temp = input_number();
    println!("You entered {temp}");
    let temp = convert_celcius_to_farenheit(temp);
    println!("Temperature in farenheit {temp}")
}
