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
        "Значение должно быть выше 0. Только целые числа",
        |&width| width > 0,
    );

    let height: u32 = input::input(
        "Введите высоту:",
        "Значение должно быть выше 0. Только целые числа",
        |&height| height > 0,
    );

    basic::Size(width, height)
}
