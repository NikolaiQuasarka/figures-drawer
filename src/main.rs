use std::io;

use rectangle::{drawing::basic, figures};

fn main() {}

fn app() {}

fn get_grid_size() -> basic::Size {
    println!("Enter grid size:");

    let width: u32 = loop {
        let mut width: String = String::new();

        if let Err(e) = io::stdin().read_line(&mut width) {
            println!("Input error: {}", e.to_string());
            continue;
        }

        match width.parse::<u32>() {
            Ok(width) => break width,
            Err(_) => {
                eprintln!("Wrong input. Try again");
                continue;
            }
        };
    };

    let height: u32 = loop {
        let mut height: String = String::new();

        if let Err(e) = io::stdin().read_line(&mut height) {
            println!("Input error: {}", e.to_string());
            continue;
        }

        match height.parse::<u32>() {
            Ok(height) => break height,
            Err(_) => {
                eprintln!("Wrong input. Try again");
                continue;
            }
        };
    };

    basic::Size(width, height)
}
