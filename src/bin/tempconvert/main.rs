use std::io;

const FAHRENHEIT_TO_CELSIUS_CONVERSTION_FACTOR: f64 = 5.0 / 9.0;

fn convert_farhrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * FAHRENHEIT_TO_CELSIUS_CONVERSTION_FACTOR
}

fn main() {
    // deg F -> deg C
    let stdin_handle = io::stdin();

    println!("Enter value in Farhrenheit to convert to Celsius.");

    loop {
        println!("Enter value to convert or q to quit.");
        let mut input = String::new();
        stdin_handle
            .read_line(&mut input)
            .expect("Couldn't read from stdin!");
        let input = input.trim(); // rust says to shadow... will take some getting used to

        if input == "q" {
            break;
        }

        // rust doesn't have a "is_numeric" method on the string, correct
        // way to check to see if it a number is to try a parse and then abort on Fail

        let input: f64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number.");
                continue;
            }
        };

        println!("{} F is {} C", input, convert_farhrenheit_to_celsius(input));
    }
}
