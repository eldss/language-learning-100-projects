use rust_convert::{c_to_f, f_to_c};
use std::{env, process};

const CELSIUS_FLAGS: [&str; 2] = ["-c", "--celsius"];
const FAHRENHEIT_FLAGS: [&str; 2] = ["-f", "--fahrenheit"];
const USAGE_STR: &str =
    "Usage: rust-convert <arg> <val>\n\tvalid args: --celsius/-c, --fahrenheit/-f";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        // Get args
        let temp_type: &str = &args[1];
        let temp_val: f64 = match &args[2].parse::<f64>() {
            Ok(value) => *value,
            Err(_) => {
                eprintln!("Problem parsing value: {}.\n{}", &args[2], USAGE_STR);
                process::exit(1);
            }
        };

        // Convert and report
        if CELSIUS_FLAGS.contains(&temp_type) {
            let fahrenheit = c_to_f(temp_val);
            println!("{:.2} degrees fahrenheit.", fahrenheit);
        } else if FAHRENHEIT_FLAGS.contains(&temp_type) {
            let celsius = f_to_c(temp_val);
            println!("{:.2} degrees celsius.", celsius);
        } else {
            eprintln!("Unexpected argument: {}.\n{}", temp_type, USAGE_STR);
            process::exit(1);
        }
    } else {
        eprintln!("Incorrect usage.\n{}", USAGE_STR);
        process::exit(1);
    }
}
