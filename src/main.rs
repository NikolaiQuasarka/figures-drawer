use std::io;

use rectangle::{
    drawing::{self, basic, grid},
    figures,
};

fn main() {
    app();
}

fn app() {
    let grid = loop {
        let grid_size = get_grid_size();

        let Ok(grid) = grid::Grid::from(grid_size) else {
            eprint!("Try again");
            continue;
        };

        break grid;
    };
}

fn get_grid_size() -> basic::Size {
    println!("Enter grid size:");

    let width: u32 = loop {
        println!("Enter width");

        let mut width: String = String::new();

        if let Err(e) = io::stdin().read_line(&mut width) {
            println!("Input error: {}", e.to_string());
            continue;
        }

        match width.trim().parse::<u32>() {
            Ok(width) => {
                if width < 1 {
                    eprintln!("Value below 1. Width should be 1 and higher");
                    continue;
                } else {
                    break width;
                }
            }
            Err(_) => {
                eprintln!("Wrong input. Try again");
                continue;
            }
        };
    };

    let height: u32 = loop {
        println!("Enter height");

        let mut height: String = String::new();

        if let Err(e) = io::stdin().read_line(&mut height) {
            println!("Input error: {}", e.to_string());
            continue;
        }

        match height.trim().parse::<u32>() {
            Ok(height) => {
                if height < 1 {
                    eprint!("Value below 1. Height shoud be 1 and higher");
                    continue;
                } else {
                    break height;
                }
            }
            Err(_) => {
                eprintln!("Wrong input. Try again");
                continue;
            }
        };
    };

    basic::Size(width, height)
}
