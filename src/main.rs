use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    let stdin_handle = io::stdin();

    println!("how many bands do you think i got");
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);
    println!("(man, i wish they didn't know i was thinking {secret_number})");

    loop {
        let guess: u8 = {
            let mut guess_string = String::new();
            stdin_handle.read_line(&mut guess_string).expect("Failed!");
            // this works the same way as kotlin scopes
            guess_string
                .trim()
                .parse()
                .expect("that ain't no number, fool!")
        };
        // in python, i would drop the guess variable by calling del. however the must "rustic" way to
        // do it is by having the name shadowed OR doing it in a scope.
        // i'm going to do it in a scope to let rust handle it; plus it's similar to how i would do it
        // in kotlin

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("ain't enough"),
            Ordering::Greater => println!("too much"),
            Ordering::Equal => {
                println!("dang u win");
                break;
            }
        }
    }
}
