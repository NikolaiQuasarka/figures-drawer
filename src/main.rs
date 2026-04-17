use rectangle::{drawing::basic, input};

fn main() {
    app();
}

fn app() {
    let mut grid = input::get_grid::get_grid();

    println!("Сетка создана!");

    let draw = input::choose_figure::get_draw().create_drawing().unwrap();

    println!("Рисунок создан");

    grid.draw(draw, basic::Point::default());

    println!("{}", grid.to_string())
}
