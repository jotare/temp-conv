use std::io::{self, Write};

fn celsius_to_farenheit(c: f64) -> f64{
    (9.0 / 5.0) * c + 32.0
}

fn farenheit_to_celsius(f: f64) -> f64 {
    (5.0 / 9.0) * (f - 32.0)
}

fn read_number(prompt: &str) -> Option<f64> {
    let mut number = String::new();

    print!("{}", prompt);
    io::stdout().flush().expect("Can't flush stdout");

    io::stdin().read_line(&mut number)
        .expect("Failed to read line");

    match number.trim().parse() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("You must write a number");
            None
        }
    }
}

fn main() {
    println!("Welcome to the temperature converter");
    println!("Write Celsius (C) or Farenheit (F) to convert. Quit (Q) to exit.");

    loop {
        print!("> ");
        io::stdout().flush().expect("Can't flush stdout");

        let mut option = String::new();
        io::stdin().read_line(&mut option)
            .expect("Failed to read line");
        let option = option.trim().to_lowercase();

        if option == "c" || option == "celsius" {
            let temp_c = match read_number("Enter Celsius: ") {
                Some(n) => n,
                None => continue,
            };

            let temp_f = celsius_to_farenheit(temp_c);
            println!("{temp_c:.2} ºC are {temp_f:.2} ºF");

        } else if option == "f" || option == "farenheit" {
            let temp_f = match read_number("Enter Farenheit: ") {
                Some(n) => n,
                None => continue
            };

            let temp_c = farenheit_to_celsius(temp_f);
            println!("{temp_f:.2} ºF are {temp_c:.2} ºC");

        } else if option == "q" || option == "quit" {
            break;

        } else {
            println!("Sorry, I don't understand it");
        }
        println!();
    }
}
