use rectangle::input::{self};

fn main() {
    app();
}

fn app() {
    let grid = input::get_grid::get_grid();

    println!("A grid is created!");

    let draw = input::choose_figure::get_draw();
}
