use std::io;

use rectangle::{
    drawing::{self, basic, grid},
    figures::{self, Drawable},
    input,
};

fn main() {
    app();
}

fn app() {
    let grid = input::get_grid::get_grid();

    println!("A grid is created!");
}
