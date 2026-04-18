use std::io;

use crate::{
    drawing::{basic, grid},
    input,
};

pub fn get_grid() -> grid::Grid {
    loop {
        let grid_size = get_grid_size();

        let Ok(grid) = grid::Grid::from(grid_size) else {
            eprint!("Try again");
            continue;
        };

        break grid;
    }
}

fn get_grid_size() -> basic::Size {
    println!("Enter grid size:");

    let width: u32 = input::input(
        "Введите ширину:",
        "Значение должно быть выше 1",
        |&width| width > 0,
    );

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
